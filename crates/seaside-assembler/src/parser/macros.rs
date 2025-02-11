macro_rules! if_enabled {
    ($self:ident, $directive:ident ($directive_repr:literal) => $callback:expr) => {
        if $self
            .special_directives
            .intersects(seaside_config::features::assembler::SpecialDirectives::$directive)
        {
            $callback
        } else {
            Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::DirectiveDisabled,
                format!("{} is disabled in the config", $directive_repr),
            ))
        }
    };
}
pub(crate) use if_enabled;

macro_rules! assert_token {
    ($self:ident, $variant:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant)) => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1)) => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident | $variant2:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1 | Token::$variant2)) => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
}
pub(crate) use assert_token;

macro_rules! assert_token_or_none {
    ($self:ident, $variant:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant)) | None => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1)) | None => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident | $variant2:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1 | Token::$variant2)) | None => {}
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
        }
    };
}
pub(crate) use assert_token_or_none;

macro_rules! token_is {
    ($self:ident, $variant:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant)) => true,
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                false
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1)) => true,
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                false
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident | $variant2:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0 | Token::$variant1 | Token::$variant2)) => true,
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                false
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
}
pub(crate) use token_is;

macro_rules! token_contents_or_err {
    ($self:ident, $variant:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant(x))) => x,
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0(x) | Token::$variant1(x))) => x,
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident | $variant2:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0(x) | Token::$variant1(x)) | Token::$variant2(x)) => x,
            Some(Ok(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnexpectedToken,
                ));
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::PrematureEof,
                ));
            }
        }
    };
}
pub(crate) use token_contents_or_err;

macro_rules! maybe_token_contents {
    ($self:ident, $variant:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant(x))) => Some(x),
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                None
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => None,
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0(x) | Token::$variant1(x))) => Some(x),
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                None
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => None,
        }
    };
    ($self:ident, $variant0:ident | $variant1:ident | $variant2:ident) => {
        match $self.next_token() {
            Some(Ok(Token::$variant0(x) | Token::$variant1(x)) | Token::$variant2(x)) => Some(x),
            Some(Ok(token)) => {
                $self.peeked.push_back(token);
                None
            }
            Some(Err(_)) => {
                return Err($crate::parser::Error::from(
                    $crate::parser::ErrorKind::UnknownToken,
                ));
            }
            None => None,
        }
    };
}
pub(crate) use maybe_token_contents;

macro_rules! get_operand {
    ($self:ident, gpr) => {
        Operand::Register(
            $crate::parser::macros::token_contents_or_err!($self, RegisterIndex | CpuRegisterName),
        )
    };
    ($self:ident, gpr?) => {{
        $crate::parser::macros::maybe_token_contents!($self, RegisterIndex | CpuRegisterName)
            .map(|__gpr__| Operand::Register(__gpr__))
    }};
    ($self:ident, fpr) => {
        Operand::Register(
            $crate::parser::macros::token_contents_or_err!($self, RegisterIndex | Cop1RegisterName),
        )
    };
    ($self:ident, fpr?) => {{
        $crate::parser::macros::maybe_token_contents!($self, RegisterIndex | Cop1RegisterName)
            .map(|__fpr__| Operand::Register(__fpr__))
    }};
    ($self:ident, exr) => {
        Operand::Register(
            $crate::parser::macros::token_contents_or_err!($self, RegisterIndex | Cop0RegisterName),
        )
    };
    ($self:ident, exr?) => {{
        $crate::parser::macros::maybe_token_contents!($self, RegisterIndex | Cop1RegisterName)
            .map(|__exr__| Operand::Register(__exr__))
    }};
    ($self:ident, wrapped_gpr) => {{
        $crate::parser::macros::assert_token!($self, LParen);
        let __gpr__ = $crate::parser::macros::token_contents_or_err!(
            $self,
            RegisterIndex | CpuRegisterName
        );
        $crate::parser::macros::assert_token!($self, RParen);
        Operand::WrappedRegister(__gpr__)
    }};
    ($self:ident, cc) => {{
        let __int__ = $crate::parser::macros::token_contents_or_err!($self, IntLiteral);
        match <i32 as TryInto<u8>>::try_into(__int__) {
            Ok(__cc__) if __cc__ < 8 => Operand::Cc(__cc__),
            _ => return Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::ValueOutsideRange,
                "cc must be on the range 0..=7",
            )),
        }
    }};
    ($self:ident, cc?) => {{
        match $crate::parser::macros::maybe_token_contents!($self, IntLiteral) {
            Some(__int__) => match <i32 as TryInto<u8>>::try_into(__int__) {
                Ok(__cc__) if __cc__ < 8 => Some(Operand::Cc(__cc__)),
                _ => return Err($crate::parser::Error::new(
                    $crate::parser::ErrorKind::ValueOutsideRange,
                    "cc must be on the range 0..=7",
                )),
            },
            None => None,
        }
    }};
    ($self:ident, shamt) => {{
        let __int__ = $crate::parser::macros::token_contents_or_err!($self, IntLiteral);
        match <i32 as TryInto<u8>>::try_into(__int__) {
            Ok(__shamt__) if __shamt__ < 32 => Operand::Shamt(__shamt__),
            _ => return Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::ValueOutsideRange,
                "shamt must be on the range 0..=31",
            )),
        }
    }};
    ($self:ident, shamt?) => {{
        match $crate::parser::macros::maybe_token_contents!($self, IntLiteral) {
            Some(__int__) match <i32 as TryInto<u8>>::try_into(__int__) {
                Ok(__shamt__) if __shamt__ < 32 => Some(Operand::Shamt(__shamt__)),
                _ => return Err($crate::parser::Error::new(
                    $crate::parser::ErrorKind::ValueOutsideRange,
                    "shamt must be on the range 0..=31",
                )),
            },
            None => None,
        }
    }};
    ($self:ident, i16) => {{
        let __int__ = $crate::parser::macros::token_contents_or_err!($self, IntLiteral);
        match <i32 as TryInto<i16>>::try_into(__int__) {
            Ok(__i16__) => Operand::I16(__i16__),
            Err(_) => return Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::ValueOutsideRange,
                "immediate must be a valid i16",
            )),
        }
    }};
    ($self:ident, i16?) => {{
        match $crate::parser::macros::maybe_token_contents!($self, IntLiteral) {
            Some(__int__) match <i32 as TryInto<i16>>::try_into(__int__) {
                Ok(__i16__) => Some(Operand::I16(__i16__)),
                Err(_) => return Err($crate::parser::Error::new(
                    $crate::parser::ErrorKind::ValueOutsideRange,
                    "immediate must be a valid i16",
                )),
            },
            None => None,
        }
    }};
    ($self:ident, u16) => {{
        let __int__ = $crate::parser::macros::token_contents_or_err!($self, IntLiteral);
        match <i32 as TryInto<u16>>::try_into(__int__) {
            Ok(__u16__) => Operand::U16(__u16__),
            Err(_) => return Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::ValueOutsideRange,
                "immediate must be a valid u16",
            )),
        }
    }};
    ($self:ident, u16?) => {{
        match $crate::parser::macros::maybe_token_contents!($self, IntLiteral) {
            Some(__int__) match <i32 as TryInto<u16>>::try_into(__int__) {
                Ok(__u16__) => Some(Operand::U16(__u16__)),
                Err(_) => return Err($crate::parser::Error::new(
                    $crate::parser::ErrorKind::ValueOutsideRange,
                    "immediate must be a valid u16",
                )),
            },
            None => None,
        }
    }};
    ($self:ident, code) => {{
        let __code__ = $crate::parser::macros::token_contents_or_err!($self, IntLiteral);
        if __code__ < (1 << 20) {
            Operand::Code(__code__)
        } else {
            return Err($crate::parser::Error::new(
                $crate::parser::ErrorKind::ValueOutsideRange,
                "code must be on the range 0..=1048575",
            ));
        }
    }};
    ($self:ident, code?) => {{
        match $crate::parser::macros::maybe_token_contents!($self, IntLiteral) {
            Some(__code__) => {
                let __code__ = __code__ as u32;
                if __code__ < (1 << 20) {
                    Some(Operand::Code(__code__))
                } else {
                    return Err($crate::parser::Error::new(
                        $crate::parser::ErrorKind::ValueOutsideRange,
                        "code must be on the range 0..=1048575",
                    ));
                }
            }
            None => None,
        }
    }};
    ($self:ident, label) => {
        Operand::Label($crate::parser::macros::token_contents_or_err!($self, Label))
    };
    ($self:ident, label?) => {
        $crate::parser::macros::maybe_token_contents!($self, Label)
            .map(|__label__| Operand::Label(__label__))
    };
}
pub(crate) use get_operand;

macro_rules! parse_ops {
    ($self:ident) => {{
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [None, None, None]
    }};
    ($self:ident, $op0:ident) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [Some(__op0__), None, None]
    }};
    ($self:ident, $op0:ident?) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0?);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [__op0__, None, None]
    }};
    ($self:ident, $op0:ident, $op1:ident) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0);
        $crate::parser::macros::assert_token!($self, Comma);
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [Some(__op0__), Some(__op1__), None]
    }};
    ($self:ident, $op0:ident?, $op1:ident) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0?);
        if __op0__.is_some() {
            $crate::parser::macros::assert_token!($self, Comma);
        }
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [__op0__, Some(__op1__), None]
    }};
    ($self:ident, $op0:ident, $op1:ident?) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0);
        let __comma_exists__ = $crate::parser::macros::token_is!($self, Comma);
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1?);
        if __op1__.is_some() ^ __comma_exists__ {
            return Err($crate::parser::Error::from(
                $crate::parser::ErrorKind::UnexpectedToken,
            ));
        }
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [Some(__op0__), __op1__, None]
    }};
    ($self:ident, $op0:ident, $op1:ident, $op2:ident) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0);
        $crate::parser::macros::assert_token!($self, Comma);
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1);
        $crate::parser::macros::assert_token!($self, Comma);
        let __op2__ = $crate::parser::macros::get_operand!($self, $op2);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [Some(__op0__), Some(__op1__), Some(__op2__)]
    }};
    ($self:ident, $op0:ident?, $op1:ident, $op2:ident) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0?);
        if __op0__.is_some() {
            $crate::parser::macros::assert_token!($self, Comma);
        }
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1);
        $crate::parser::macros::assert_token!($self, Comma);
        let __op2__ = $crate::parser::macros::get_operand!($self, $op2);
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [__op0__, Some(__op1__), Some(__op2__)]
    }};
    ($self:ident, $op0:ident, $op1:ident, $op2:ident?) => {{
        let __op0__ = $crate::parser::macros::get_operand!($self, $op0);
        $crate::parser::macros::assert_token!($self, Comma);
        let __op1__ = $crate::parser::macros::get_operand!($self, $op1);
        let __comma_exists__ = $crate::parser::macros::token_is!($self, Comma);
        let __op2__ = $crate::parser::macros::get_operand!($self, $op2?);
        if __op2__.is_some() ^ __comma_exists__ {
            return Err($crate::parser::Error::from(
                $crate::parser::ErrorKind::UnexpectedToken,
            ));
        }
        $crate::parser::macros::assert_token_or_none!($self, NewLine);
        [Some(__op0__), Some(__op1__), __op2__]
    }};
}
pub(crate) use parse_ops;
