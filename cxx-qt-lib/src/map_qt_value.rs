// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::let_qcolor;
use crate::let_qstring;
use crate::let_qvariant;
use crate::qcolor::{Color, QColor};
use crate::qpointf::QPointF;
use crate::qsizef::QSizeF;
use crate::qstring::QString;
use crate::qvariant::{QVariant, Variant};

pub trait MapQtValue<C, F, R> {
    fn map_qt_value(&self, map_func: F, context: &mut C) -> R;
}

impl<C, R> MapQtValue<C, fn(&mut C, &QColor) -> R, R> for Color {
    fn map_qt_value(&self, map_func: fn(&mut C, &QColor) -> R, context: &mut C) -> R {
        let_qcolor!(c = self);
        map_func(context, &c)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QString) -> R, R> for &str {
    fn map_qt_value(&self, map_func: fn(&mut C, &QString) -> R, context: &mut C) -> R {
        let_qstring!(s = self);
        map_func(context, &s)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QString) -> R, R> for String {
    fn map_qt_value(&self, map_func: fn(&mut C, &QString) -> R, context: &mut C) -> R {
        let_qstring!(s = self);
        map_func(context, &s)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QVariant) -> R, R> for Variant {
    fn map_qt_value(&self, map_func: fn(&mut C, &QVariant) -> R, context: &mut C) -> R {
        let_qvariant!(v = self);
        map_func(context, &v)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QPointF) -> R, R> for QPointF {
    fn map_qt_value(&self, map_func: fn(&mut C, &QPointF) -> R, context: &mut C) -> R {
        map_func(context, self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QSizeF) -> R, R> for QSizeF {
    fn map_qt_value(&self, map_func: fn(&mut C, &QSizeF) -> R, context: &mut C) -> R {
        map_func(context, self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, bool) -> R, R> for bool {
    fn map_qt_value(&self, map_func: fn(&mut C, bool) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, f32) -> R, R> for f32 {
    fn map_qt_value(&self, map_func: fn(&mut C, f32) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, f64) -> R, R> for f64 {
    fn map_qt_value(&self, map_func: fn(&mut C, f64) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, i8) -> R, R> for i8 {
    fn map_qt_value(&self, map_func: fn(&mut C, i8) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, i16) -> R, R> for i16 {
    fn map_qt_value(&self, map_func: fn(&mut C, i16) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, i32) -> R, R> for i32 {
    fn map_qt_value(&self, map_func: fn(&mut C, i32) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, i64) -> R, R> for i64 {
    fn map_qt_value(&self, map_func: fn(&mut C, i64) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, u8) -> R, R> for u8 {
    fn map_qt_value(&self, map_func: fn(&mut C, u8) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, u16) -> R, R> for u16 {
    fn map_qt_value(&self, map_func: fn(&mut C, u16) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, u32) -> R, R> for u32 {
    fn map_qt_value(&self, map_func: fn(&mut C, u32) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, u64) -> R, R> for u64 {
    fn map_qt_value(&self, map_func: fn(&mut C, u64) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}