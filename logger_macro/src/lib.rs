extern crate proc_macro;
use proc_macro::*;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn initialize(input: TokenStream, anno: TokenStream) -> TokenStream {
    let res = 
    if let TokenTree::Group(g) = anno.clone().into_iter().nth(14).unwrap() {
        let args = g.stream();
        let names: Vec<_> = args.clone().into_iter().zip(args.clone().into_iter().skip(1)).filter_map(|(a, b)| {
            if let TokenTree::Ident(i) = a {
                if let TokenTree::Punct(p) = b {
                    if p == ':' && p.spacing() == Spacing::Alone { return Some(TokenTree::Ident(i)); }
                }
            }
            return None;
        }).collect();
        let mut va: Vec<_> = anno.clone().into_iter().collect();
        let lena = va.len();
        va[lena - 1] = 
        if let TokenTree::Group(g1) = anno.clone().into_iter().last().unwrap() {
            let mut v1: Vec<_> = g1.stream().into_iter().collect();
            let len1 = v1.len();
            v1[len1 - 1] = 
            if let TokenTree::Group(g2) = g1.stream().into_iter().last().unwrap() {
                let mut v2: Vec<_> = g2.stream().into_iter().collect();
                let len2 = v2.len();
                v2[len2 - 1] = 
                if let TokenTree::Group(g3) = g2.stream().into_iter().last().unwrap() {
                    let mut v: Vec<_> = g3.stream().into_iter().collect();
                    let len = v.len();
                    let code = v[len - 5].clone();
                    let start0 = TokenStream::from_str(r#"let __l = format!"#).unwrap();
                    let mut start1 = TokenStream::from_str(r#""{:?}","#).unwrap();
                    let mut vs: Vec<_> = start1.into_iter().collect();
                    vs.push(names[1].clone());
                    start1 = TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vs.into_iter()))).into();
                    let start2 = TokenStream::from_str(r#";
                        let __a = linera_sdk::contract::system_api::current_application_id();
                        let __i ="#).unwrap();
                    let start3 = TokenStream::from_str(r#";
                        let __n = find_crate::find_crate(|s| true)?.name;
                        let __b = "#).unwrap();
                    let start4 = TokenStream::from_str(r#".height;
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::InitializationStart,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        let __r ="#).unwrap();
                    let end = TokenStream::from_str(r#";
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::InitializationEnd,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        __r"#).unwrap();
                    let mut replacement = TokenStream::new();
                    replacement.extend(start0.into_iter());
                    replacement.extend(start1.into_iter());
                    replacement.extend(start2.into_iter());
                    replacement.extend(input.clone().into_iter());
                    replacement.extend(start3.into_iter());
                    replacement.extend(vec![names[0].clone()].into_iter());
                    replacement.extend(start4.into_iter());
                    replacement.extend(vec![code].into_iter());
                    replacement.extend(end.into_iter());
                    v[len - 5] = TokenTree::Group(Group::new(Delimiter::Brace, replacement));
                    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v.into_iter())))
                } else { TokenTree::Literal(Literal::string("incorrect format")) };
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(v2.into_iter())))
            } else { TokenTree::Literal(Literal::string("incorrect format")) };
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v1.into_iter())))
        } else { TokenTree::Literal(Literal::string("incorrect format")) };
        TokenStream::from_iter(va.into_iter())
    } else { TokenStream::from_str(r#""incorrect format""#).unwrap() };
    //panic!("{}", res);
    res
}

#[proc_macro_attribute]
pub fn execute_operation(input: TokenStream, anno: TokenStream) -> TokenStream {
    let res = 
    if let TokenTree::Group(g) = anno.clone().into_iter().nth(14).unwrap() {
        let args = g.stream();
        let names: Vec<_> = args.clone().into_iter().zip(args.clone().into_iter().skip(1)).filter_map(|(a, b)| {
            if let TokenTree::Ident(i) = a {
                if let TokenTree::Punct(p) = b {
                    if p == ':' && p.spacing() == Spacing::Alone { return Some(TokenTree::Ident(i)); }
                }
            }
            return None;
        }).collect();
        let mut va: Vec<_> = anno.clone().into_iter().collect();
        let lena = va.len();
        va[lena - 1] = 
        if let TokenTree::Group(g1) = anno.clone().into_iter().last().unwrap() {
            let mut v1: Vec<_> = g1.stream().into_iter().collect();
            let len1 = v1.len();
            v1[len1 - 1] = 
            if let TokenTree::Group(g2) = g1.stream().into_iter().last().unwrap() {
                let mut v2: Vec<_> = g2.stream().into_iter().collect();
                let len2 = v2.len();
                v2[len2 - 1] = 
                if let TokenTree::Group(g3) = g2.stream().into_iter().last().unwrap() {
                    let mut v: Vec<_> = g3.stream().into_iter().collect();
                    let len = v.len();
                    let code = v[len - 5].clone();
                    let start0 = TokenStream::from_str(r#"let __l = format!"#).unwrap();
                    let mut start1 = TokenStream::from_str(r#""{:?}","#).unwrap();
                    let mut vs: Vec<_> = start1.into_iter().collect();
                    vs.push(names[1].clone());
                    start1 = TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vs.into_iter()))).into();
                    let start2 = TokenStream::from_str(r#";
                        let __a = linera_sdk::contract::system_api::current_application_id();
                        let __i ="#).unwrap();
                    let start3 = TokenStream::from_str(r#";
                        let __n = find_crate::find_crate(|s| true)?.name;
                        let __b = "#).unwrap();
                    let start4 = TokenStream::from_str(r#".height;
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::OperationExecutionStart,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        let __r ="#).unwrap();
                    let end = TokenStream::from_str(r#";
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::OperationExecutionEnd,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        __r"#).unwrap();
                    let mut replacement = TokenStream::new();
                    replacement.extend(start0.into_iter());
                    replacement.extend(start1.into_iter());
                    replacement.extend(start2.into_iter());
                    replacement.extend(input.clone().into_iter());
                    replacement.extend(start3.into_iter());
                    replacement.extend(vec![names[0].clone()].into_iter());
                    replacement.extend(start4.into_iter());
                    replacement.extend(vec![code].into_iter());
                    replacement.extend(end.into_iter());
                    v[len - 5] = TokenTree::Group(Group::new(Delimiter::Brace, replacement));
                    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v.into_iter())))
                } else { TokenTree::Literal(Literal::string("incorrect format")) };
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(v2.into_iter())))
            } else { TokenTree::Literal(Literal::string("incorrect format")) };
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v1.into_iter())))
        } else { TokenTree::Literal(Literal::string("incorrect format")) };
        TokenStream::from_iter(va.into_iter())
    } else { TokenStream::from_str(r#""incorrect format""#).unwrap() };
    //panic!("{}", res);
    res
}


#[proc_macro_attribute]
pub fn execute_message(input: TokenStream, anno: TokenStream) -> TokenStream {
    let res = 
    if let TokenTree::Group(g) = anno.clone().into_iter().nth(14).unwrap() {
        let args = g.stream();
        let names: Vec<_> = args.clone().into_iter().zip(args.clone().into_iter().skip(1)).filter_map(|(a, b)| {
            if let TokenTree::Ident(i) = a {
                if let TokenTree::Punct(p) = b {
                    if p == ':' && p.spacing() == Spacing::Alone { return Some(TokenTree::Ident(i)); }
                }
            }
            return None;
        }).collect();
        let mut va: Vec<_> = anno.clone().into_iter().collect();
        let lena = va.len();
        va[lena - 1] = 
        if let TokenTree::Group(g1) = anno.clone().into_iter().last().unwrap() {
            let mut v1: Vec<_> = g1.stream().into_iter().collect();
            let len1 = v1.len();
            v1[len1 - 1] = 
            if let TokenTree::Group(g2) = g1.stream().into_iter().last().unwrap() {
                let mut v2: Vec<_> = g2.stream().into_iter().collect();
                let len2 = v2.len();
                v2[len2 - 1] = 
                if let TokenTree::Group(g3) = g2.stream().into_iter().last().unwrap() {
                    let mut v: Vec<_> = g3.stream().into_iter().collect();
                    let len = v.len();
                    let code = v[len - 5].clone();
                    let start0 = TokenStream::from_str(r#"let __l = format!"#).unwrap();
                    let mut start1 = TokenStream::from_str(r#""{:?}","#).unwrap();
                    let mut vs: Vec<_> = start1.into_iter().collect();
                    vs.push(names[1].clone());
                    start1 = TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vs.into_iter()))).into();
                    let start2 = TokenStream::from_str(r#";
                        let __a = linera_sdk::contract::system_api::current_application_id();
                        let __i ="#).unwrap();
                    let start3 = TokenStream::from_str(r#";
                        let __n = find_crate::find_crate(|s| true)?.name;
                        let __b = "#).unwrap();
                    let start4 = TokenStream::from_str(r#".height;
                        let __o = "#).unwrap();
                    let start5 = TokenStream::from_str(r#".message_id.chain_id;
                        let __f = "#).unwrap();
                    let start6 = TokenStream::from_str(r#".message_id.height;
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::MessageExecutionStart,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: __o,
                            from_block_height: __f,
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        let __r ="#).unwrap();
                    let end = TokenStream::from_str(r#";
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::MessageExecutionEnd,
                            log: __l.clone(),
                            block_height: __b,
                            other_chain: __o,
                            from_block_height: __f,
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        __r"#).unwrap();
                    let mut replacement = TokenStream::new();
                    replacement.extend(start0.into_iter());
                    replacement.extend(start1.into_iter());
                    replacement.extend(start2.into_iter());
                    replacement.extend(input.clone().into_iter());
                    replacement.extend(start3.into_iter());
                    replacement.extend(vec![names[0].clone()].into_iter());
                    replacement.extend(start4.into_iter());
                    replacement.extend(vec![names[0].clone()].into_iter());
                    replacement.extend(start5.into_iter());
                    replacement.extend(vec![names[0].clone()].into_iter());
                    replacement.extend(start6.into_iter());
                    replacement.extend(vec![code].into_iter());
                    replacement.extend(end.into_iter());
                    v[len - 5] = TokenTree::Group(Group::new(Delimiter::Brace, replacement));
                    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v.into_iter())))
                } else { TokenTree::Literal(Literal::string("incorrect format")) };
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(v2.into_iter())))
            } else { TokenTree::Literal(Literal::string("incorrect format")) };
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v1.into_iter())))
        } else { TokenTree::Literal(Literal::string("incorrect format")) };
        TokenStream::from_iter(va.into_iter())
    } else { TokenStream::from_str(r#""incorrect format""#).unwrap() };
    //panic!("{}", res);
    res
}

#[proc_macro_attribute]
pub fn handle_application_call(input: TokenStream, anno: TokenStream) -> TokenStream {
    let res = 
    if let TokenTree::Group(g) = anno.clone().into_iter().nth(14).unwrap() {
        let args = g.stream();
        let names: Vec<_> = args.clone().into_iter().zip(args.clone().into_iter().skip(1)).filter_map(|(a, b)| {
            if let TokenTree::Ident(i) = a {
                if let TokenTree::Punct(p) = b {
                    if p == ':' && p.spacing() == Spacing::Alone { return Some(TokenTree::Ident(i)); }
                }
            }
            return None;
        }).collect();
        let mut va: Vec<_> = anno.clone().into_iter().collect();
        let lena = va.len();
        va[lena - 1] = 
        if let TokenTree::Group(g1) = anno.clone().into_iter().last().unwrap() {
            let mut v1: Vec<_> = g1.stream().into_iter().collect();
            let len1 = v1.len();
            v1[len1 - 1] = 
            if let TokenTree::Group(g2) = g1.stream().into_iter().last().unwrap() {
                let mut v2: Vec<_> = g2.stream().into_iter().collect();
                let len2 = v2.len();
                v2[len2 - 1] = 
                if let TokenTree::Group(g3) = g2.stream().into_iter().last().unwrap() {
                    let mut v: Vec<_> = g3.stream().into_iter().collect();
                    let len = v.len();
                    let code = v[len - 5].clone();
                    let start0 = TokenStream::from_str(r#"let __l = format!"#).unwrap();
                    let mut start1 = TokenStream::from_str(r#""{:?}","#).unwrap();
                    let mut vs: Vec<_> = start1.into_iter().collect();
                    vs.push(names[1].clone());
                    start1 = TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vs.into_iter()))).into();
                    let start2 = TokenStream::from_str(r#";
                        let __a = linera_sdk::contract::system_api::current_application_id();
                        let __i ="#).unwrap();
                    let start3 = TokenStream::from_str(r#";
                        let __n = find_crate::find_crate(|s| true)?.name;
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::ApplicationCallHandleStart,
                            log: __l.clone(),
                            block_height: linera_sdk::base::BlockHeight(0),
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        let __r ="#).unwrap();
                    let end = TokenStream::from_str(r#";
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::ApplicationCallHandleEnd,
                            log: __l.clone(),
                            block_height: linera_sdk::base::BlockHeight(0),
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        __r"#).unwrap();
                    let mut replacement = TokenStream::new();
                    replacement.extend(start0.into_iter());
                    replacement.extend(start1.into_iter());
                    replacement.extend(start2.into_iter());
                    replacement.extend(input.clone().into_iter());
                    replacement.extend(start3.into_iter());
                    replacement.extend(vec![code].into_iter());
                    replacement.extend(end.into_iter());
                    v[len - 5] = TokenTree::Group(Group::new(Delimiter::Brace, replacement));
                    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v.into_iter())))
                } else { TokenTree::Literal(Literal::string("incorrect format")) };
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(v2.into_iter())))
            } else { TokenTree::Literal(Literal::string("incorrect format")) };
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v1.into_iter())))
        } else { TokenTree::Literal(Literal::string("incorrect format")) };
        TokenStream::from_iter(va.into_iter())
    } else { TokenStream::from_str(r#""incorrect format""#).unwrap() };
    //panic!("{}", res);
    res
}

#[proc_macro_attribute]
pub fn handle_session_call(input: TokenStream, anno: TokenStream) -> TokenStream {
    let res = 
    if let TokenTree::Group(g) = anno.clone().into_iter().nth(14).unwrap() {
        let args = g.stream();
        let names: Vec<_> = args.clone().into_iter().zip(args.clone().into_iter().skip(1)).filter_map(|(a, b)| {
            if let TokenTree::Ident(i) = a {
                if let TokenTree::Punct(p) = b {
                    if p == ':' && p.spacing() == Spacing::Alone { return Some(TokenTree::Ident(i)); }
                }
            }
            return None;
        }).collect();
        let mut va: Vec<_> = anno.clone().into_iter().collect();
        let lena = va.len();
        va[lena - 1] = 
        if let TokenTree::Group(g1) = anno.clone().into_iter().last().unwrap() {
            let mut v1: Vec<_> = g1.stream().into_iter().collect();
            let len1 = v1.len();
            v1[len1 - 1] = 
            if let TokenTree::Group(g2) = g1.stream().into_iter().last().unwrap() {
                let mut v2: Vec<_> = g2.stream().into_iter().collect();
                let len2 = v2.len();
                v2[len2 - 1] = 
                if let TokenTree::Group(g3) = g2.stream().into_iter().last().unwrap() {
                    let mut v: Vec<_> = g3.stream().into_iter().collect();
                    let len = v.len();
                    let code = v[len - 5].clone();
                    let start0 = TokenStream::from_str(r#"let __l = format!"#).unwrap();
                    let mut start1 = TokenStream::from_str(r#""{:?}","#).unwrap();
                    let mut vs: Vec<_> = start1.into_iter().collect();
                    vs.push(names[2].clone());
                    start1 = TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vs.into_iter()))).into();
                    let start2 = TokenStream::from_str(r#";
                        let __a = linera_sdk::contract::system_api::current_application_id();
                        let __i ="#).unwrap();
                    let start3 = TokenStream::from_str(r#";
                        let __n = find_crate::find_crate(|s| true)?.name;
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::SessionCallHandleStart,
                            log: __l.clone(),
                            block_height: linera_sdk::base::BlockHeight(0),
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        let __r ="#).unwrap();
                    let end = TokenStream::from_str(r#";
                        __self.call_application(true, __i, &logger::LogStatement {
                            log_type: logger::LogType::SessionCallHandleEnd,
                            log: __l.clone(),
                            block_height: linera_sdk::base::BlockHeight(0),
                            other_chain: linera_sdk::base::ChainId::root(0),
                            from_block_height: linera_sdk::base::BlockHeight(0),
                            app: __a,
                            app_name: __n.clone(),
                        }, vec![]).await?;
                        __r"#).unwrap();
                    let mut replacement = TokenStream::new();
                    replacement.extend(start0.into_iter());
                    replacement.extend(start1.into_iter());
                    replacement.extend(start2.into_iter());
                    replacement.extend(input.clone().into_iter());
                    replacement.extend(start3.into_iter());
                    replacement.extend(vec![code].into_iter());
                    replacement.extend(end.into_iter());
                    v[len - 5] = TokenTree::Group(Group::new(Delimiter::Brace, replacement));
                    TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v.into_iter())))
                } else { TokenTree::Literal(Literal::string("incorrect format")) };
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(v2.into_iter())))
            } else { TokenTree::Literal(Literal::string("incorrect format")) };
            TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(v1.into_iter())))
        } else { TokenTree::Literal(Literal::string("incorrect format")) };
        TokenStream::from_iter(va.into_iter())
    } else { TokenStream::from_str(r#""incorrect format""#).unwrap() };
    //panic!("{}", res);
    res
}