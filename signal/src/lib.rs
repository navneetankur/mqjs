use core::ffi::c_int;

use rquickjs::{atom::PredefinedAtom, class::{OwnedBorrow, OwnedBorrowMut}, Ctx, Function, Object, Value};
use signal_hook::{consts as sigconsts, iterator::{exfiltrator::SignalOnly, Pending, Signals}};

pub fn add_sig_types(types: &Object) {
    types.set("sigint", sigconsts::SIGINT).unwrap();
    types.set("sigterm", sigconsts::SIGTERM).unwrap();
}
pub fn add_signal_props(global: &Object) {
    let api = global.get("api").unwrap_or_else(|_| Object::new(global.ctx().clone()).unwrap());
    let signal = api.get("signal").unwrap_or_else(|_| Object::new(api.ctx().clone()).unwrap());
    common::object_fn!(signal,
        receiver,
        receiver_sigint,
        receiver_sigusr1,
        receiver_allterm,
    );
    let types = signal.get("types").unwrap_or_else(|_| Object::new(api.ctx().clone()).unwrap());
    add_sig_types(&types);
    signal.set("types", types).unwrap();
    api.set("signal", signal).unwrap();
    global.set("api", api).unwrap();
}
fn receiver_sigint() -> SignalReciever {
    let sigint = Signals::new([sigconsts::SIGINT]).unwrap();
    return SignalReciever::new(sigint);
}
fn receiver_sigusr1() -> SignalReciever {
    let sigusr1 = Signals::new([sigconsts::SIGUSR1]).unwrap();
    return SignalReciever::new(sigusr1);
}
fn receiver_allterm() -> SignalReciever {
    let term_sigs = Signals::new(sigconsts::TERM_SIGNALS).unwrap();
    return SignalReciever::new(term_sigs);
}
fn receiver(value: Value) -> SignalReciever {
    if let Some(signal) = value.as_int() {
        let sr = Signals::new([signal as c_int]).unwrap();
        return sr.into();
    }
    else if let Some(signals) = value.into_array() {
        let signals_vec: Vec<c_int> = signals.iter::<c_int>().map(|v| v.expect("only int signals allowed")).collect();
        return Signals::new(signals_vec).unwrap().into();
    }
    panic!("only int or array of int allowed.");
}
common::class_chore!(SignalReciever, get_proto);
pub struct SignalReciever {
    v: Signals,
}
impl From<Signals> for SignalReciever {
    fn from(value: Signals) -> Self {
        Self::new(value)
    }
}
impl SignalReciever {
    fn new(v: Signals) -> Self {
        Self { v }
    }
}
type ThisMut<'js> = rquickjs::function::This<OwnedBorrowMut<'js, SignalReciever>>;
type This<'js> = rquickjs::function::This<OwnedBorrow<'js, SignalReciever>>;
// type This<'js> = rquickjs::function::This<Class<'js, SignalReciever>>;
fn iterator(mut this: ThisMut) -> common::iterator::JsIterator<Pending<SignalOnly>> {
    this.v.pending().into()
}
fn add_signal(this: This, signal: c_int) {
    this.v.add_signal(signal).unwrap();
}
fn get_proto<'js>(ctx: &Ctx<'js>) -> Object<'js> {
    let proto = Object::new(ctx.clone()).unwrap();
    proto.set(PredefinedAtom::SymbolIterator, 
        Function::new(ctx.clone(), iterator).unwrap()
    ).unwrap();
    common::object_fn!(proto, 
        add_signal,
        iterator,
    );
    return proto;
}
