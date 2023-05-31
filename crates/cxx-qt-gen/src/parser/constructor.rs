// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, Error, GenericArgument, ItemImpl, Path,
    PathArguments, PathSegment, Result, Type,
};

#[derive(Default)]
struct ConstructorArguments {
    /// Arguments to the new function.
    /// The `new` function needs to return the inner Rust struct for the QObject.
    new: Option<Vec<Type>>,
    /// Arguments to be passed to the base class constructor.
    base: Option<Vec<Type>>,
    /// Arguments to the initialize function.
    /// The `initialize` function is run after the QObject is created.
    initialize: Option<Vec<Type>>,
}

/// A parsed cxx_qt::Constructor trait impl.
pub struct Constructor {
    /// The arguments to the constructor defined by this trait impl.
    pub arguments: Vec<Type>,

    /// Arguments to the new function.
    /// The `new` function needs to return the inner Rust struct for the QObject.
    pub new_arguments: Option<Vec<Type>>,
    /// Arguments to be passed to the base class constructor.
    pub base_arguments: Option<Vec<Type>>,
    /// Arguments to the initialize function.
    /// The `initialize` function is run after the QObject is created.
    pub initialize_arguments: Option<Vec<Type>>,

    /// The original impl that this constructor was parse from.
    pub imp: ItemImpl,
}

impl Constructor {
    fn parse_argument_list(ty: Type) -> Result<Vec<Type>> {
        Ok(match ty {
            Type::Tuple(tuple) => tuple.elems.into_iter().collect(),
            _ => return Err(Error::new(ty.span(), "Expected a tuple as argument list!\nNote that a tuple of a single type needs to use a trailing comma, e.g. (i32,)"))
        })
    }

    fn parse_associated_types(generics: &[&GenericArgument]) -> Result<ConstructorArguments> {
        let mut arguments = ConstructorArguments::default();
        for generic in generics {
            if let GenericArgument::AssocType(associated) = generic {
                let argument_list = match &*associated.ident.to_string() {
                    "NewArguments" => &mut arguments.new,
                    "InitializeArguments" => &mut arguments.initialize,
                    "BaseArguments" => &mut arguments.base,
                    _ => return Err(Error::new_spanned(generic, "Unknown associated type!")),
                };
                if argument_list.is_some() {
                    return Err(Error::new_spanned(
                        generic,
                        "Duplicate associated type definition!",
                    ));
                }

                *argument_list = Some(Self::parse_argument_list(associated.ty.clone())?);
            } else {
                return Err(Error::new_spanned(
                    generic,
                    "Expected associated type as a generic argument!",
                ));
            }
        }
        Ok(arguments)
    }

    fn parse_generics(
        trait_path: &Path,
        generics: &[&GenericArgument],
    ) -> Result<(Vec<Type>, ConstructorArguments)> {
        if let Some((GenericArgument::Type(arguments_tuple), generics)) = generics.split_first() {
            let argument_types = Self::parse_argument_list(arguments_tuple.clone())?;

            let arguments = Self::parse_associated_types(generics)?;

            Ok((argument_types, arguments))
        } else {
            let span = if generics.is_empty() {
                trait_path.span()
            } else {
                generics[0].span()
            };

            Err(Error::new(
                span,
                "cxx_qt::Constructor expects a tuple as the first generic argument",
            ))
        }
    }

    fn parse_arguments(trait_path: &Path) -> Result<(Vec<Type>, ConstructorArguments)> {
        let constructor_path: Vec<_> = trait_path.segments.iter().collect();
        if let [
            // cxx_qt::
            _cxx_qt,
            // Constructor
            PathSegment {
            arguments:
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args: generics_punct, .. }),
            ..
        }] = *constructor_path
        {
            let generics: Vec<_> = generics_punct.iter().collect();

            Self::parse_generics(trait_path, &generics)
        } else {
            Err(Error::new_spanned(
                trait_path,
                "Missing generic argument for cxx_qt::Constructor!",
            ))
        }
    }

    pub fn parse(imp: ItemImpl) -> Result<Self> {
        if let Some(unsafety) = imp.unsafety {
            return Err(Error::new_spanned(
                unsafety,
                "Unnecessary unsafe around constructor impl.",
            ));
        }

        if !imp.generics.params.is_empty() {
            return Err(Error::new_spanned(
                imp.generics.params,
                "Generics are not allowed on cxx_qt::Constructor impls!",
            ));
        }

        if !imp.items.is_empty() {
            return Err(Error::new_spanned(
                imp.items.first(),
                "cxx_qt::Constructor must only be declared, not implemented inside cxx_qt::bridge!",
            ));
        }

        let (_, trait_path, _) = &imp
            .trait_
            .as_ref()
            .ok_or_else(|| Error::new_spanned(imp.clone(), "Expected trait impl!"))?;

        let (argument_list, arguments) = Self::parse_arguments(trait_path)?;
        Ok(Constructor {
            arguments: argument_list,
            new_arguments: arguments.new,
            base_arguments: arguments.base,
            initialize_arguments: arguments.initialize,
            imp,
        })
    }
}