//
// Copyright (c) 2022 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fmt;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Not;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyModifiers(u32);

impl KeyModifiers
{
    pub const EMPTY: KeyModifiers = KeyModifiers(0);
    pub const SHIFT: KeyModifiers = KeyModifiers(1 << 0);
    pub const CAPS: KeyModifiers = KeyModifiers(1 << 1);
    pub const CTRL: KeyModifiers = KeyModifiers(1 << 2);
    pub const ALT: KeyModifiers = KeyModifiers(1 << 3);
    pub const NUM: KeyModifiers = KeyModifiers(1 << 4);
    pub const LOGO: KeyModifiers = KeyModifiers(1 << 5);

    pub const fn empty() -> Self
    { KeyModifiers(0) }

    pub const fn all() -> Self
    { KeyModifiers(63) }
    
    pub const fn is_empty(&self) -> bool
    { self.0 == 0 }
    
    pub const fn is_all(&self) -> bool
    { self.0 == 63 }

    pub const fn intersects(&self, other: Self) -> bool
    { self.0 & other.0 != 0 }
    
    pub const fn contains(&self, other: Self) -> bool
    { (self.0 & other.0) == other.0 }

    pub fn insert(&mut self, other: Self)
    { self.0 |= other.0; }

    pub fn remove(&mut self, other: Self)
    { self.0 &= !other.0; }

    pub fn toggle(&mut self, other: Self)
    { self.0 ^= other.0; }

    pub fn set(&mut self, other: Self, b: bool)
    {
        if b {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    
    pub fn intersection(self, other: Self) -> Self
    { KeyModifiers(self.0 & other.0) }

    pub fn union(self, other: Self) -> Self
    { KeyModifiers(self.0 | other.0) }

    pub fn difference(self, other: Self) -> Self
    { KeyModifiers(self.0 & !other.0) }

    pub fn symmetric_difference(self, other: Self) -> Self
    { KeyModifiers(self.0 ^ other.0) }
    
    pub fn complement(self) -> Self
    { KeyModifiers(self.0 ^ 63) }
}

impl Not for KeyModifiers
{
    type Output = Self;
    
    fn not(self) -> Self::Output
    { KeyModifiers(self.0 ^ 63) }
}

impl BitAnd for KeyModifiers
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output
    { KeyModifiers(self.0 & rhs.0) }
}

impl BitAndAssign for KeyModifiers
{
    fn bitand_assign(&mut self, rhs: Self)
    { self.0 &= rhs.0; }
}

impl BitOr for KeyModifiers
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output
    { KeyModifiers(self.0 | rhs.0) }
}

impl BitOrAssign for KeyModifiers
{
    fn bitor_assign(&mut self, rhs: Self)
    { self.0 |= rhs.0; }
}

impl BitXor for KeyModifiers
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output
    { KeyModifiers(self.0 ^ rhs.0) }
}

impl BitXorAssign for KeyModifiers
{
    fn bitxor_assign(&mut self, rhs: Self)
    { self.0 ^= rhs.0; }
}

impl Sub for KeyModifiers
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    { KeyModifiers(self.0 & !rhs.0) }
}

impl SubAssign for KeyModifiers
{
    fn sub_assign(&mut self, rhs: Self)
    { self.0 &= !rhs.0; }
}

impl fmt::Debug for KeyModifiers
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if *self == Self::EMPTY {
            write!(f, "EMPTY")?;
        } else {
            let mut is_first = true;
            if (*self & Self::SHIFT) != Self::EMPTY {
                write!(f, "SHIFT")?;
                is_first = false;
            }
            if (*self & Self::CAPS) != Self::EMPTY {
                if !is_first {
                    write!(f, " | ")?;
                }
                write!(f, "CAPS")?;
                is_first = false;
            }
            if (*self & Self::CTRL) != Self::EMPTY {
                if !is_first {
                    write!(f, " | ")?;
                }
                write!(f, "CTRL")?;
                is_first = false;
            }
            if (*self & Self::ALT) != Self::EMPTY {
                if !is_first {
                    write!(f, " | ")?;
                }
                write!(f, "ALT")?;
                is_first = false;
            }
            if (*self & Self::NUM) != Self::EMPTY {
                if !is_first {
                    write!(f, " | ")?;
                }
                write!(f, "NUM")?;
                is_first = false;
            }
            if (*self & Self::LOGO) != Self::EMPTY {
                if !is_first {
                    write!(f, " | ")?;
                }
                write!(f, "LOGO")?;
            }
        }
        Ok(())
    }
}

// Most names of these enumeration variants are from xkbcommon/xkbcommon-keysyms.h.

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VKey
{
    // TTY keys.
    Backspace,
    Tab,
    Linefeed,
    Clear,
    Return,
    Pause,
    ScrollLock,
    SysReq,
    Escape,
    Delete,
    // Japanse keys.
    Kanji,
    Muhenkan,
    HenkanMode,
    Henkan,
    Romaji,
    Hiragana,
    Katakana,
    HiraganaKatakana,
    Zenkaku,
    Hankaku,
    ZenkakuHankaku,
    Touroku,
    Massyo,
    KanaLock,
    KanaShift,
    EisuShift,
    EisuToggle,
    KanjiBangou,
    ZenKoho,
    MaeKoho,
    // Cursor control and motion keys.
    Home,
    Left,
    Up,
    Right,
    Down,
    Prior,
    PageUp,
    Next,
    PageDown,
    End,
    Begin,
    // Miscellaneous keys.
    Select,
    Print,
    Execute,
    Insert,
    Undo,
    Redo,
    Menu,
    Find,
    Cancel,
    Help,
    Break,
    ModeSwitch,
    ScriptSwitch,
    NumLock,
    // Keypad keys.
    KeypadSpace,
    KeypadTab,
    KeypadEnter,
    KeypadEqual,
    KeypadMultiply,
    KeypadAdd,
    KeypadSeparator,
    KeypadSubtract,
    KeypadDecimal,
    KeypadDivide,
    Kaypad0,
    Kaypad1,
    Kaypad2,
    Kaypad3,
    Kaypad4,
    Kaypad5,
    Kaypad6,
    Kaypad7,
    Kaypad8,
    Kaypad9,
    // Function keys.
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    // Modifiers.
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    CapsLock,
    ShiftLock,
    LeftMeta,
    RightMeta,
    LeftAlt,
    RightAlt,
    LeftSuper,
    RightSuper,
    LeftHyper,
    RightHyper,
    // Extension function and modifier keys.
    DeadGrave,
    DeadAcute,
    DeadCircumflex,
    DeadTilde,
    DeadPerispomeni,
    DeadMacron,
    DeadBreve,
    DeadAbovedot,
    DeadDiaeresis,
    DeadAbovering,
    DeadDoubleacute,
    DeadCaron,
    DeadCedilla,
    DeadOgonek,
    DeadIota,
    DeadVoicedSound,
    DeadSemivoicedSound,
    DeadBelowdot,
    DeadHook,
    DeadHorn,
    DeadStroke,
    DeadAbovecomma,
    DeadPsili,
    DeadAbovereversedcomma,
    DeadDasia,
    DeadDoublegrave,
    DeadBelowring,
    DeadBelowmacron,
    DeadBelowcircumflex,
    DeadBelowtilde,
    DeadBelowbreve,
    DeadBelowdiaeresis,
    DeadInvertedbreve,
    DeadBelowcomma,
    DeadCurrency,
    DeadLowline,
    DeadAboveverticalline,
    DeadBelowverticalline,
    DeadLongsolidusoverlay,
    DeadA,
    DeadE,
    DeadI,
    DeadO,
    DeadU,
    DeadSchwa,
    DeadGreek,
    // ASCII keys.
    Space,
    Exclam,
    DoubleQuote,
    NumberSign,
    Dollar,
    Percent,
    Ampersand,
    Apostrophe,
    RightQuote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Colon,
    Semicolon,
    Less,
    Equal,
    Greater,
    Question,
    At,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    Circnum,
    Underscore,
    Grave,
    LeftQuote,
    LeftBrace,
    Bar,
    RightBrace,
    Tidle,
}
