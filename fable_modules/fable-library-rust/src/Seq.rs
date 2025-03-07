#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use crate::import_eae1ac5e::*;
use crate::import_3bd9ae6a::*;
#[path = "./Interfaces.rs"]
pub(crate) mod import_971078fd;
pub use import_971078fd::*;
use crate::import_8d7d6be8::*;
use crate::import_ec6ee4e9::*;
use crate::import_f222008f::*;
use crate::import_c6216f2::*;
pub mod Seq {
    use super::*;
    pub mod SR {
        use super::*;
        pub fn enumerationAlreadyFinished() -> Rc<str> {
            thread_local! {
                static enumerationAlreadyFinished: Rc<str> =
    String_::string(&"Enumeration already finished.");
            };
            enumerationAlreadyFinished.with(|value| value.clone())
        }
        pub fn enumerationNotStarted() -> Rc<str> {
            thread_local! {
                static enumerationNotStarted: Rc<str> =
    String_::string(&"Enumeration has not started. Call MoveNext.");
            };
            enumerationNotStarted.with(|value| value.clone())
        }
        pub fn inputSequenceEmpty() -> Rc<str> {
            thread_local! {
                static inputSequenceEmpty: Rc<str> =
    String_::string(&"The input sequence was empty.");
            };
            inputSequenceEmpty.with(|value| value.clone())
        }
        pub fn inputSequenceTooLong() -> Rc<str> {
            thread_local! {
                static inputSequenceTooLong: Rc<str> =
    String_::string(&"The input sequence contains more than one element.");
            };
            inputSequenceTooLong.with(|value| value.clone())
        }
        pub fn keyNotFoundAlt() -> Rc<str> {
            thread_local! {
                static keyNotFoundAlt: Rc<str> =
    String_::string(&"An index satisfying the predicate was not found in the collection.");
            };
            keyNotFoundAlt.with(|value| value.clone())
        }
        pub fn notEnoughElements() -> Rc<str> {
            thread_local! {
                static notEnoughElements: Rc<str> =
    String_::string(&"The input sequence has an insufficient number of elements.");
            };
            notEnoughElements.with(|value| value.clone())
        }
        pub fn resetNotSupported() -> Rc<str> {
            thread_local! {
                static resetNotSupported: Rc<str> =
    String_::string(&"Reset is not supported on this enumerator.");
            };
            resetNotSupported.with(|value| value.clone())
        }
    }
    pub fn indexNotFound<a_: Clone + 'static>() -> a_ {
        panic!("{}", Seq::SR::keyNotFoundAlt())
    }
    pub mod Enumerable {
        use super::*;
        pub fn noReset<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::resetNotSupported())
        }
        pub fn notStarted<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::enumerationNotStarted())
        }
        pub fn alreadyFinished<a_: Clone + 'static>() -> a_ {
            panic!("{}", Seq::SR::enumerationAlreadyFinished())
        }
        #[derive(Clone)]
        pub struct Enumerable_1<T: Clone + 'static> {
            f: Rc<dyn Fn() -> (Rc<dyn Interfaces::IEnumerator_1<T>>) +
                  'static>,
        }
        impl <T: Clone + 'static> Seq::Enumerable::Enumerable_1<T> {
            pub fn _ctor__Z32F335EB(f:
                                        &Rc<impl Fn()
                                            ->
                                                (Rc<dyn Interfaces::IEnumerator_1<T>>) +
                                            'static>)
             -> Rc<Seq::Enumerable::Enumerable_1<T>> {
                let f_1;
                ();
                f_1 = f.clone();
                ();
                Rc::from(Seq::Enumerable::Enumerable_1::<T>{f: f_1.clone(),})
            }
        }
        impl <T: Clone + 'static> Interfaces::IEnumerable_1<T> for
         Enumerable_1<T> {
            fn GetEnumerator(&self) -> Rc<dyn Interfaces::IEnumerator_1<T>> {
                (self.f)()
            }
        }
        #[derive(Clone)]
        pub struct Enumerator<T: Clone + 'static> {
            next: Rc<dyn Fn() -> (Option<T>) + 'static>,
            curr: MutCell<Option<T>>,
        }
        impl <T: Clone + 'static> Seq::Enumerable::Enumerator<T> {
            pub fn _ctor__29DEF9BC(next:
                                       &Rc<impl Fn() -> (Option<T>) +
                                           'static>)
             -> Rc<Seq::Enumerable::Enumerator<T>> {
                let next_1;
                let curr: Option<T>;
                ();
                next_1 = next.clone();
                curr = None::<T>;
                ();
                Rc::from(Seq::Enumerable::Enumerator::<T>{next:
                                                              next_1.clone(),
                                                          curr:
                                                              MutCell::from(curr.clone()),})
            }
        }
        impl <T: Clone + 'static> Interfaces::IEnumerator_1<T> for
         Enumerator<T> {
            fn Current(&self) -> T {
                Option_::getValue(&self.curr.get()).clone()
            }
            fn MoveNext(&self) -> bool {
                self.curr.set((self.next)());
                self.curr.get().is_some()
            }
            fn Dispose(&self) { (); }
        }
        pub fn fromFunction<T: Clone +
                            'static>(next:
                                         &Rc<impl Fn() -> (Option<T>) +
                                             'static>)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            Seq::Enumerable::Enumerator::_ctor__29DEF9BC(next).clone() as
                Rc<dyn Interfaces::IEnumerator_1<T>>
        }
        pub fn empty<T: Clone + 'static>()
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            Seq::Enumerable::fromFunction(&Rc::from(move || None::<T>))
        }
        pub fn singleton<T: Clone + 'static>(x: &T)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let i = i.clone();
                                                        let x = x.clone();
                                                        move ||
                                                            if i.get() < 1i32
                                                               {
                                                                i.set(i.get()
                                                                          +
                                                                          1i32);
                                                                Some(x.clone())
                                                            } else {
                                                                None::<T>
                                                            }
                                                    }))
        }
        pub fn ofArray<T: Clone + 'static>(arr: &Rc<MutCell<Vec<T>>>)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let len: i32 = arr.len() as i32;
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let arr = arr.clone();
                                                        let i = i.clone();
                                                        move ||
                                                            if i.get() < len {
                                                                i.set(i.get()
                                                                          +
                                                                          1i32);
                                                                Some(arr[i.get()
                                                                             -
                                                                             1i32].clone())
                                                            } else {
                                                                None::<T>
                                                            }
                                                    }))
        }
        pub fn ofList<T: Clone + 'static>(xs: &List_1<T>)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let it: Rc<MutCell<List_1<T>>> =
                Rc::from(MutCell::from(xs.clone()));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let it = it.clone();
                                                        move ||
                                                            if !List_::isEmpty(&it.get())
                                                               {
                                                                let tail_1:
                                                                        List_1<T> =
                                                                    List_::tail(&it.get()).clone();
                                                                let head_1:
                                                                        T =
                                                                    List_::head(&it.get()).clone();
                                                                it.set(tail_1);
                                                                Some(head_1)
                                                            } else {
                                                                None::<T>
                                                            }
                                                    }))
        }
        pub fn append<T: Clone +
                      'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                               ys: &Rc<dyn Interfaces::IEnumerable_1<T>>)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(-1i32));
            let innerOpt:
                    Rc<MutCell<Option<Rc<dyn Interfaces::IEnumerator_1<T>>>>> =
                Rc::from(MutCell::from(None::<Rc<dyn Interfaces::IEnumerator_1<T>>>));
            let finished: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let finished =
                                                            finished.clone();
                                                        let i = i.clone();
                                                        let innerOpt =
                                                            innerOpt.clone();
                                                        let xs = xs.clone();
                                                        let ys = ys.clone();
                                                        move ||
                                                            {
                                                                let moveNext:
                                                                        Rc<MutCell<bool>> =
                                                                    Rc::from(MutCell::from(false));
                                                                while if !finished.get()
                                                                         {
                                                                          !moveNext.get()
                                                                      } else {
                                                                          false
                                                                      } {
                                                                    match &innerOpt.get()
                                                                        {
                                                                        None
                                                                        =>
                                                                        if i.get()
                                                                               <
                                                                               1i32
                                                                           {
                                                                            i.set(i.get()
                                                                                      +
                                                                                      1i32);
                                                                            {
                                                                                let it:
                                                                                        Rc<dyn Interfaces::IEnumerable_1<T>> =
                                                                                    if i.get()
                                                                                           ==
                                                                                           0i32
                                                                                       {
                                                                                        xs.clone()
                                                                                    } else {
                                                                                        ys.clone()
                                                                                    };
                                                                                innerOpt.set(Some(it.GetEnumerator()))
                                                                            }
                                                                        } else {
                                                                            finished.set(true)
                                                                        },
                                                                        Some(innerOpt_0_0)
                                                                        => {
                                                                            let inner:
                                                                                    Rc<dyn Interfaces::IEnumerator_1<T>> =
                                                                                innerOpt_0_0.clone();
                                                                            if inner.MoveNext()
                                                                               {
                                                                                moveNext.set(true)
                                                                            } else {
                                                                                innerOpt.set(None::<Rc<dyn Interfaces::IEnumerator_1<T>>>)
                                                                            }
                                                                        }
                                                                    };
                                                                }
                                                                if if !finished.get()
                                                                      {
                                                                       moveNext.get()
                                                                   } else {
                                                                       false
                                                                   } {
                                                                    Some(Option_::getValue(&innerOpt.get()).Current().clone())
                                                                } else {
                                                                    None::<T>
                                                                }
                                                            }
                                                    }))
        }
        pub fn concat<T: Clone +
                      'static>(sources:
                                   &Rc<dyn Interfaces::IEnumerable_1<Rc<dyn Interfaces::IEnumerable_1<T>>>>)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let outerOpt:
                    Rc<MutCell<Option<Rc<dyn Interfaces::IEnumerator_1<Rc<dyn Interfaces::IEnumerable_1<T>>>>>>> =
                Rc::from(MutCell::from(None::<Rc<dyn Interfaces::IEnumerator_1<Rc<dyn Interfaces::IEnumerable_1<T>>>>>));
            let innerOpt:
                    Rc<MutCell<Option<Rc<dyn Interfaces::IEnumerator_1<T>>>>> =
                Rc::from(MutCell::from(None::<Rc<dyn Interfaces::IEnumerator_1<T>>>));
            let finished: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let finished =
                                                            finished.clone();
                                                        let innerOpt =
                                                            innerOpt.clone();
                                                        let outerOpt =
                                                            outerOpt.clone();
                                                        let sources =
                                                            sources.clone();
                                                        move ||
                                                            {
                                                                let moveNext:
                                                                        Rc<MutCell<bool>> =
                                                                    Rc::from(MutCell::from(false));
                                                                while if !finished.get()
                                                                         {
                                                                          !moveNext.get()
                                                                      } else {
                                                                          false
                                                                      } {
                                                                    match &outerOpt.get()
                                                                        {
                                                                        None
                                                                        =>
                                                                        outerOpt.set(Some(sources.GetEnumerator())),
                                                                        Some(outerOpt_0_0)
                                                                        => {
                                                                            let outer:
                                                                                    Rc<dyn Interfaces::IEnumerator_1<Rc<dyn Interfaces::IEnumerable_1<T>>>> =
                                                                                outerOpt_0_0.clone();
                                                                            match &innerOpt.get()
                                                                                {
                                                                                None
                                                                                =>
                                                                                if outer.MoveNext()
                                                                                   {
                                                                                    let it:
                                                                                            Rc<dyn Interfaces::IEnumerable_1<T>> =
                                                                                        outer.Current().clone();
                                                                                    innerOpt.set(Some(it.GetEnumerator()))
                                                                                } else {
                                                                                    finished.set(true)
                                                                                },
                                                                                Some(innerOpt_0_0)
                                                                                =>
                                                                                {
                                                                                    let inner:
                                                                                            Rc<dyn Interfaces::IEnumerator_1<T>> =
                                                                                        innerOpt_0_0.clone();
                                                                                    if inner.MoveNext()
                                                                                       {
                                                                                        moveNext.set(true)
                                                                                    } else {
                                                                                        innerOpt.set(None::<Rc<dyn Interfaces::IEnumerator_1<T>>>)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                }
                                                                if if !finished.get()
                                                                      {
                                                                       moveNext.get()
                                                                   } else {
                                                                       false
                                                                   } {
                                                                    Some(Option_::getValue(&innerOpt.get()).Current().clone())
                                                                } else {
                                                                    None::<T>
                                                                }
                                                            }
                                                    }))
        }
        pub fn generateWhileSome<T: Clone + 'static, U: Clone +
                                 'static>(openf:
                                              &Rc<impl Fn() -> (T) + 'static>,
                                          compute:
                                              &Rc<impl Fn(&T) -> (Option<U>) +
                                                  'static>,
                                          closef: &Rc<impl Fn(&T) + 'static>)
         -> Rc<dyn Interfaces::IEnumerator_1<U>> {
            let finished: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
            let state: Rc<MutCell<Option<T>>> =
                Rc::from(MutCell::from(None::<T>));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let closef =
                                                            closef.clone();
                                                        let compute =
                                                            compute.clone();
                                                        let finished =
                                                            finished.clone();
                                                        let openf =
                                                            openf.clone();
                                                        let state =
                                                            state.clone();
                                                        move ||
                                                            if finished.get()
                                                               {
                                                                None::<U>
                                                            } else {
                                                                if state.get().is_none()
                                                                   {
                                                                    state.set(Some(openf()));
                                                                }
                                                                {
                                                                    let res:
                                                                            Option<U> =
                                                                        compute(&Option_::getValue(&state.get()));
                                                                    if res.is_none()
                                                                       {
                                                                        finished.set(true);
                                                                        match &state.get()
                                                                            {
                                                                            Some(state_0_0)
                                                                            =>
                                                                            {
                                                                                let x:
                                                                                        T =
                                                                                    state_0_0.clone();
                                                                                {
                                                                                    let try_result =
                                                                                        closef(&x);
                                                                                    state.set(None::<T>);
                                                                                    try_result
                                                                                }
                                                                            }
                                                                            _
                                                                            =>
                                                                            (),
                                                                        }
                                                                    }
                                                                    res.clone()
                                                                }
                                                            }
                                                    }))
        }
        pub fn unfold<State: Clone + 'static, T: Clone +
                      'static>(f:
                                   &Rc<impl Fn(&State)
                                       -> (Option<(T, State)>) + 'static>,
                               state: &State)
         -> Rc<dyn Interfaces::IEnumerator_1<T>> {
            let acc: Rc<MutCell<State>> =
                Rc::from(MutCell::from(state.clone()));
            Seq::Enumerable::fromFunction(&Rc::from({
                                                        let acc = acc.clone();
                                                        let f = f.clone();
                                                        move ||
                                                            {
                                                                let matchValue:
                                                                        Option<(T,
                                                                                State)> =
                                                                    f(&acc.get());
                                                                match &matchValue
                                                                    {
                                                                    None =>
                                                                    None::<T>,
                                                                    Some(matchValue_0_0)
                                                                    => {
                                                                        let x:
                                                                                T =
                                                                            matchValue_0_0.0.clone();
                                                                        let st:
                                                                                State =
                                                                            matchValue_0_0.1.clone();
                                                                        acc.set(st);
                                                                        Some(x)
                                                                    }
                                                                }
                                                            }
                                                    }))
        }
    }
    pub fn mkSeq<T: Clone +
                 'static>(f:
                              &Rc<impl Fn()
                                  -> (Rc<dyn Interfaces::IEnumerator_1<T>>) +
                                  'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::Enumerable::Enumerable_1::_ctor__Z32F335EB(f).clone() as
            Rc<dyn Interfaces::IEnumerable_1<T>>
    }
    pub fn ofSeq<T: Clone +
                 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerator_1<T>> {
        xs.GetEnumerator()
    }
    pub fn delay<T: Clone +
                 'static>(generator:
                              &Rc<impl Fn()
                                  -> (Rc<dyn Interfaces::IEnumerable_1<T>>) +
                                  'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let generator = generator.clone();
                                 move || generator().GetEnumerator()
                             }))
    }
    pub fn concat<T: Clone +
                  'static>(sources:
                               &Rc<dyn Interfaces::IEnumerable_1<Rc<dyn Interfaces::IEnumerable_1<T>>>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let sources = sources.clone();
                                 move || Seq::Enumerable::concat(&sources)
                             }))
    }
    pub fn unfold<State: Clone + 'static, T: Clone +
                  'static>(generator:
                               &Rc<impl Fn(&State) -> (Option<(T, State)>) +
                                   'static>, state: &State)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let generator = generator.clone();
                                 let state = state.clone();
                                 move ||
                                     Seq::Enumerable::unfold(&generator,
                                                             &state)
                             }))
    }
    pub fn empty<a_: Clone + 'static>()
     -> Rc<dyn Interfaces::IEnumerable_1<a_>> {
        Seq::mkSeq(&Rc::from(move || Seq::Enumerable::empty()))
    }
    pub fn singleton<T: Clone + 'static>(x: &T)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let x = x.clone();
                                 move || Seq::Enumerable::singleton(&x)
                             }))
    }
    pub fn ofArray<T: Clone + 'static>(arr: &Rc<MutCell<Vec<T>>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let arr = arr.clone();
                                 move || Seq::Enumerable::ofArray(&arr)
                             }))
    }
    pub fn ofList<T: Clone + 'static>(xs: &List_1<T>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let xs = xs.clone();
                                 move || Seq::Enumerable::ofList(&xs)
                             }))
    }
    pub fn generate<a_: Clone + 'static, b_: Clone +
                    'static>(create: &Rc<impl Fn() -> (a_) + 'static>,
                             compute:
                                 &Rc<impl Fn(&a_) -> (Option<b_>) + 'static>,
                             dispose: &Rc<impl Fn(&a_) + 'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<b_>> {
        Seq::mkSeq(&Rc::from({
                                 let compute = compute.clone();
                                 let create = create.clone();
                                 let dispose = dispose.clone();
                                 move ||
                                     Seq::Enumerable::generateWhileSome(&create,
                                                                        &compute,
                                                                        &dispose)
                             }))
    }
    pub fn generateIndexed<a_: Clone + 'static, b_: Clone +
                           'static>(create: &Rc<impl Fn() -> (a_) + 'static>,
                                    compute:
                                        &Rc<impl Fn(&i32, &a_)
                                            -> (Option<b_>) + 'static>,
                                    dispose: &Rc<impl Fn(&a_) + 'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<b_>> {
        Seq::mkSeq(&Rc::from({
                                 let compute = compute.clone();
                                 let create = create.clone();
                                 let dispose = dispose.clone();
                                 move ||
                                     {
                                         let i: Rc<MutCell<i32>> =
                                             Rc::from(MutCell::from(-1i32));
                                         Seq::Enumerable::generateWhileSome(&create,
                                                                            &Rc::from({
                                                                                          let compute
                                                                                              =
                                                                                              compute.clone();
                                                                                          let i
                                                                                              =
                                                                                              i.clone();
                                                                                          move
                                                                                              |x:
                                                                                                   &a_|
                                                                                              {
                                                                                                  i.set(i.get()
                                                                                                            +
                                                                                                            1i32);
                                                                                                  compute(&i.get(),
                                                                                                          x)
                                                                                              }
                                                                                      }),
                                                                            &dispose)
                                     }
                             }))
    }
    pub fn append<T: Clone +
                  'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                           ys: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let xs = xs.clone();
                                 let ys = ys.clone();
                                 move || Seq::Enumerable::append(&xs, &ys)
                             }))
    }
    pub fn choose<T: Clone + 'static, U: Clone +
                  'static>(chooser: &Rc<impl Fn(&T) -> (Option<U>) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let chooser = chooser.clone();
                                    move
                                        |e:
                                             &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                        {
                                            let curr: Rc<MutCell<Option<U>>> =
                                                Rc::from(MutCell::from(None::<U>));
                                            while if curr.get().is_none() {
                                                      e.MoveNext()
                                                  } else { false } {
                                                curr.set(chooser(&e.Current()));
                                            }
                                            curr.get().clone()
                                        }
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                    e_1.Dispose()))
    }
    pub fn compareWith<T: Clone +
                       'static>(comparer:
                                    &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                                xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                                ys: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> i32 {
        let e1: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let e2: Rc<dyn Interfaces::IEnumerator_1<T>> =
                        Seq::ofSeq(ys);
                    {
                        let try_result_1 =
                            {
                                let c: Rc<MutCell<i32>> =
                                    Rc::from(MutCell::from(0i32));
                                let b1: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(e1.MoveNext()));
                                let b2: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(e2.MoveNext()));
                                while if if c.get() == 0i32 {
                                             b1.get()
                                         } else { false } {
                                          b2.get()
                                      } else { false } {
                                    c.set(comparer(&e1.Current(),
                                                   &e2.Current()));
                                    if c.get() == 0i32 {
                                        b1.set(e1.MoveNext());
                                        b2.set(e2.MoveNext())
                                    }
                                }
                                if c.get() != 0i32 {
                                    c.get()
                                } else {
                                    if b1.get() {
                                        1i32
                                    } else {
                                        if b2.get() { -1i32 } else { 0i32 }
                                    }
                                }
                            };
                        if false { e2.Dispose(); }
                        try_result_1
                    }
                };
            if false { e1.Dispose(); }
            try_result
        }
    }
    pub fn compareTo<T: PartialOrd + Clone +
                     'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                              ys: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> i32 {
        Seq::compareWith(&Rc::from(move |e1: &T, e2: &T|
                                       Util::compare(e1, e2)), xs, ys)
    }
    pub fn equalsTo<T: PartialOrd + Clone +
                    'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                             ys: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> bool {
        Seq::compareTo(xs, ys) == 0i32
    }
    pub fn exactlyOne<T: Clone +
                      'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> T {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                if e.MoveNext() {
                    let v: T = e.Current().clone();
                    if e.MoveNext() {
                        panic!("{}",
                               Rc::from((Rc::from(Seq::SR::inputSequenceTooLong().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
                    } else { v }
                } else {
                    panic!("{}",
                           Rc::from((Rc::from(Seq::SR::inputSequenceEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>)
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn tryExactlyOne<T: Clone +
                         'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                if e.MoveNext() {
                    let v: T = e.Current().clone();
                    if e.MoveNext() { None::<T> } else { Some(v) }
                } else { None::<T> };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn exists<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> bool {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let found: Rc<MutCell<bool>> =
                        Rc::from(MutCell::from(false));
                    while if !found.get() { e.MoveNext() } else { false } {
                        found.set(predicate(&e.Current()));
                    }
                    found.get()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn exists2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                            ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> bool {
        let e1: Rc<dyn Interfaces::IEnumerator_1<T1>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let e2: Rc<dyn Interfaces::IEnumerator_1<T2>> =
                        Seq::ofSeq(ys);
                    {
                        let try_result_1 =
                            {
                                let found: Rc<MutCell<bool>> =
                                    Rc::from(MutCell::from(false));
                                while if if !found.get() {
                                             e1.MoveNext()
                                         } else { false } {
                                          e2.MoveNext()
                                      } else { false } {
                                    found.set(predicate(&e1.Current(),
                                                        &e2.Current()));
                                }
                                found.get()
                            };
                        if false { e2.Dispose(); }
                        try_result_1
                    }
                };
            if false { e1.Dispose(); }
            try_result
        }
    }
    pub fn contains<T: Eq + core::hash::Hash + Clone +
                    'static>(value: &T,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> bool {
        Seq::exists(&Rc::from({
                                  let value = value.clone();
                                  move |x: &T| x.clone() == value.clone()
                              }), xs)
    }
    pub fn filter<T: Clone +
                  'static>(f: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::choose(&Rc::from({
                                  let f = f.clone();
                                  move |x: &T|
                                      if f(x) {
                                          Some(x.clone())
                                      } else { None::<T> }
                              }), xs)
    }
    pub fn tryFind<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let res: Rc<MutCell<Option<T>>> =
                        Rc::from(MutCell::from(None::<T>));
                    while if res.get().is_none() {
                              e.MoveNext()
                          } else { false } {
                        let c: T = e.Current().clone();
                        if predicate(&c) { res.set(Some(c.clone())); }
                    }
                    res.get().clone()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn find<T: Clone +
                'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        let matchValue: Option<T> = Seq::tryFind(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndex<T: Clone +
                        'static>(predicate:
                                     &Rc<impl Fn(&T) -> (bool) + 'static>,
                                 xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<i32> {
        fn inner_loop<T: Clone +
                      'static>(i: &i32,
                               predicate_1:
                                   &Rc<impl Fn(&T) -> (bool) + 'static>,
                               e: &Rc<dyn Interfaces::IEnumerator_1<T>>)
         -> Option<i32> {
            let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(i.clone()));
            let predicate_1 = Rc::from(MutCell::from(predicate_1.clone()));
            let e: Rc<MutCell<Rc<dyn Interfaces::IEnumerator_1<T>>>> =
                Rc::from(MutCell::from(e.clone()));
            '_inner_loop:
                loop  {
                    break '_inner_loop
                        (if e.get().MoveNext() {
                             if predicate_1.get()(&e.Current()) {
                                 Some(i.get())
                             } else {
                                 let i_temp: i32 = i.get() + 1i32;
                                 let predicate_1_temp = predicate_1.get();
                                 let e_temp:
                                         Rc<dyn Interfaces::IEnumerator_1<T>> =
                                     e.get();
                                 i.set(i_temp);
                                 predicate_1.set(predicate_1_temp);
                                 e.set(e_temp);
                                 continue '_inner_loop
                             }
                         } else { None::<i32> }) ;
                }
        }
        let e_1: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result = inner_loop(&0i32, predicate, &e_1);
            if false { e_1.Dispose(); }
            try_result
        }
    }
    pub fn findIndex<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> i32 {
        let matchValue: Option<i32> = Seq::tryFindIndex(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn fold<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> State {
        let acc: Rc<MutCell<State>> = Rc::from(MutCell::from(state.clone()));
        {
            let enumerator: Rc<dyn Interfaces::IEnumerator_1<T>> =
                xs.GetEnumerator();
            {
                let try_result =
                    while enumerator.MoveNext() {
                        let x: T = enumerator.Current().clone();
                        acc.set(folder(&acc.get(), &x))
                    };
                if false { enumerator.Dispose(); }
                try_result
            }
        }
        acc.get().clone()
    }
    pub fn toArray<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<MutCell<Vec<T>>> {
        let res: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        {
            let enumerator: Rc<dyn Interfaces::IEnumerator_1<T>> =
                xs.GetEnumerator();
            {
                let try_result =
                    while enumerator.MoveNext() {
                        let x: T = enumerator.Current().clone();
                        res.get_mut().push(x)
                    };
                if false { enumerator.Dispose(); }
                try_result
            }
        }
        res
    }
    pub fn toList<T: Clone +
                  'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> List_1<T> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                List_::unfold(&Rc::from(move
                                            |e_1:
                                                 &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                            if e_1.MoveNext() {
                                                Some((e_1.Current().clone(),
                                                      e_1.clone()))
                                            } else {
                                                None::<(T,
                                                        Rc<dyn Interfaces::IEnumerator_1<T>>)>
                                            }), &e);
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn foldBack<T: Clone + 'static, a_: Clone +
                    'static>(folder: &Rc<impl Fn(&T, &a_) -> (a_) + 'static>,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                             state: &a_) -> a_ {
        Array_::foldBack(folder, &Seq::toArray(xs), state)
    }
    pub fn fold2<State: Clone + 'static, T1: Clone + 'static, T2: Clone +
                 'static>(folder:
                              &Rc<impl Fn(&State, &T1, &T2) -> (State) +
                                  'static>, state: &State,
                          xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                          ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> State {
        let e1: Rc<dyn Interfaces::IEnumerator_1<T1>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let e2: Rc<dyn Interfaces::IEnumerator_1<T2>> =
                        Seq::ofSeq(ys);
                    {
                        let try_result_1 =
                            {
                                let acc: Rc<MutCell<State>> =
                                    Rc::from(MutCell::from(state.clone()));
                                while if e1.MoveNext() {
                                          e2.MoveNext()
                                      } else { false } {
                                    acc.set(folder(&acc.get(), &e1.Current(),
                                                   &e2.Current()));
                                }
                                acc.get().clone()
                            };
                        if false { e2.Dispose(); }
                        try_result_1
                    }
                };
            if false { e1.Dispose(); }
            try_result
        }
    }
    pub fn foldBack2<T1: Clone + 'static, T2: Clone + 'static, State: Clone +
                     'static>(folder:
                                  &Rc<impl Fn(&T1, &T2, &State) -> (State) +
                                      'static>,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                              ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>,
                              state: &State) -> State {
        Array_::foldBack2(folder, &Seq::toArray(xs), &Seq::toArray(ys), state)
    }
    pub fn forAll<T: Clone +
                  'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> bool {
        !Seq::exists(&Rc::from({
                                   let predicate = predicate.clone();
                                   move |x: &T| !predicate(x)
                               }), xs)
    }
    pub fn forAll2<T1: Clone + 'static, T2: Clone +
                   'static>(predicate:
                                &Rc<impl Fn(&T1, &T2) -> (bool) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                            ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> bool {
        !Seq::exists2(&Rc::from({
                                    let predicate = predicate.clone();
                                    move |x: &T1, y: &T2| !predicate(x, y)
                                }), xs, ys)
    }
    pub fn tryFindBack<T: Clone +
                       'static>(predicate:
                                    &Rc<impl Fn(&T) -> (bool) + 'static>,
                                xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        Array_::tryFindBack(predicate, &Seq::toArray(xs))
    }
    pub fn findBack<T: Clone +
                    'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        let matchValue: Option<T> = Seq::tryFindBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryFindIndexBack<T: Clone +
                            'static>(predicate:
                                         &Rc<impl Fn(&T) -> (bool) + 'static>,
                                     xs:
                                         &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<i32> {
        Array_::tryFindIndexBack(predicate, &Seq::toArray(xs))
    }
    pub fn findIndexBack<T: Clone +
                         'static>(predicate:
                                      &Rc<impl Fn(&T) -> (bool) + 'static>,
                                  xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> i32 {
        let matchValue: Option<i32> = Seq::tryFindIndexBack(predicate, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn tryHead<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                if e.MoveNext() {
                    Some(e.Current().clone())
                } else { None::<T> };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn head<T: Clone + 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> T {
        let matchValue: Option<T> = Seq::tryHead(xs);
        match &matchValue {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(Seq::SR::inputSequenceEmpty().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn initialize<a_: Clone +
                      'static>(count: &i32,
                               f: &Rc<impl Fn(&i32) -> (a_) + 'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<a_>> {
        Seq::unfold(&Rc::from({
                                  let count = count.clone();
                                  let f = f.clone();
                                  move |i: &i32|
                                      if i.clone() < count {
                                          Some((f(i), i.clone() + 1i32))
                                      } else { None::<(a_, i32)> }
                              }), &0i32)
    }
    pub fn initializeInfinite<a_: Clone +
                              'static>(f:
                                           &Rc<impl Fn(&i32) -> (a_) +
                                               'static>)
     -> Rc<dyn Interfaces::IEnumerable_1<a_>> {
        Seq::initialize(&i32::MAX, f)
    }
    pub fn isEmpty<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> bool {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result = !e.MoveNext();
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn tryItem<T: Clone +
                   'static>(index: &i32,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        let i: Rc<MutCell<i32>> = Rc::from(MutCell::from(index.clone()));
        if i.get() < 0i32 {
            None::<T>
        } else {
            let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
            {
                let try_result =
                    {
                        while if i.get() >= 0i32 {
                                  e.MoveNext()
                              } else { false } {
                            i.set(i.get() - 1i32);
                        }
                        if i.get() >= 0i32 {
                            None::<T>
                        } else { Some(e.Current().clone()) }
                    };
                if false { e.Dispose(); }
                try_result
            }
        }
    }
    pub fn item<T: Clone +
                'static>(index: &i32,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        let matchValue: Option<T> = Seq::tryItem(index, xs);
        match &matchValue {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"index")) as Rc<str>),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn iterate<T: Clone +
                   'static>(action: &Rc<impl Fn(&T) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) {
        Seq::fold(&Rc::from({
                                let action = action.clone();
                                move |unitVar0: &(), x: &T| action(x)
                            }), &(), xs);
    }
    pub fn iterate2<T1: Clone + 'static, T2: Clone +
                    'static>(action: &Rc<impl Fn(&T1, &T2) + 'static>,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                             ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>) {
        Seq::fold2(&Rc::from({
                                 let action = action.clone();
                                 move |unitVar0: &(), x: &T1, y: &T2|
                                     action(x, y)
                             }), &(), xs, ys);
    }
    pub fn iterateIndexed<T: Clone +
                          'static>(action: &Rc<impl Fn(&i32, &T) + 'static>,
                                   xs:
                                       &Rc<dyn Interfaces::IEnumerable_1<T>>) {
        Util::ignore(&Seq::fold(&Rc::from({
                                              let action = action.clone();
                                              move |i: &i32, x: &T|
                                                  {
                                                      action(i, x);
                                                      i.clone() + 1i32
                                                  }
                                          }), &0i32, xs));
    }
    pub fn iterateIndexed2<T1: Clone + 'static, T2: Clone +
                           'static>(action:
                                        &Rc<impl Fn(&i32, &T1, &T2) +
                                            'static>,
                                    xs:
                                        &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                                    ys:
                                        &Rc<dyn Interfaces::IEnumerable_1<T2>>) {
        Util::ignore(&Seq::fold2(&Rc::from({
                                               let action = action.clone();
                                               move |i: &i32, x: &T1, y: &T2|
                                                   {
                                                       action(i, x, y);
                                                       i.clone() + 1i32
                                                   }
                                           }), &0i32, xs, ys));
    }
    pub fn tryLast<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<T> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                if e.MoveNext() {
                    let acc: Rc<MutCell<T>> =
                        Rc::from(MutCell::from(e.Current().clone()));
                    while e.MoveNext() { acc.set(e.Current().clone()); }
                    Some(acc.get().clone())
                } else { None::<T> };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn last<T: Clone + 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> T {
        let matchValue: Option<T> = Seq::tryLast(xs);
        match &matchValue {
            None =>
            panic!("{}",
                   Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn length<T: Clone +
                  'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> i32 {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    while e.MoveNext() { count.set(count.get() + 1i32); }
                    count.get()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn map<T: Clone + 'static, U: Clone +
               'static>(mapping: &Rc<impl Fn(&T) -> (U) + 'static>,
                        xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |e:
                                             &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                        if e.MoveNext() {
                                            Some(mapping(&e.Current()))
                                        } else { None::<U> }
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                    e_1.Dispose()))
    }
    pub fn mapIndexed<T: Clone + 'static, U: Clone +
                      'static>(mapping:
                                   &Rc<impl Fn(&i32, &T) -> (U) + 'static>,
                               xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let mapping = mapping.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                               if e.MoveNext() {
                                                   Some(mapping(i,
                                                                &e.Current()))
                                               } else { None::<U> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                           e_1.Dispose()))
    }
    pub fn indexed<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(i32, T)>> {
        Seq::mapIndexed(&Rc::from(move |i: &i32, x: &T|
                                      (i.clone(), x.clone())), xs)
    }
    pub fn map2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                'static>(mapping: &Rc<impl Fn(&T1, &T2) -> (U) + 'static>,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    let ys = ys.clone();
                                    move || (Seq::ofSeq(&xs), Seq::ofSeq(&ys))
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                               Rc<dyn Interfaces::IEnumerator_1<T2>>)|
                                        {
                                            let e1:
                                                    Rc<dyn Interfaces::IEnumerator_1<T1>> =
                                                tupledArg.0.clone();
                                            let e2:
                                                    Rc<dyn Interfaces::IEnumerator_1<T2>> =
                                                tupledArg.1.clone();
                                            if if e1.MoveNext() {
                                                   e2.MoveNext()
                                               } else { false } {
                                                Some(mapping(&e1.Current(),
                                                             &e2.Current()))
                                            } else { None::<U> }
                                        }
                                }),
                      &Rc::from(move
                                    |tupledArg_1:
                                         &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                           Rc<dyn Interfaces::IEnumerator_1<T2>>)|
                                    {
                                        let try_result =
                                            tupledArg_1.0.clone().Dispose();
                                        tupledArg_1.1.clone().Dispose();
                                        try_result
                                    }))
    }
    pub fn mapIndexed2<T1: Clone + 'static, T2: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&i32, &T1, &T2) -> (U) +
                                        'static>,
                                xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                                ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           let ys = ys.clone();
                                           move ||
                                               (Seq::ofSeq(&xs),
                                                Seq::ofSeq(&ys))
                                       }),
                             &Rc::from({
                                           let mapping = mapping.clone();
                                           move
                                               |i: &i32,
                                                tupledArg:
                                                    &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                                      Rc<dyn Interfaces::IEnumerator_1<T2>>)|
                                               {
                                                   let e1:
                                                           Rc<dyn Interfaces::IEnumerator_1<T1>> =
                                                       tupledArg.0.clone();
                                                   let e2:
                                                           Rc<dyn Interfaces::IEnumerator_1<T2>> =
                                                       tupledArg.1.clone();
                                                   if if e1.MoveNext() {
                                                          e2.MoveNext()
                                                      } else { false } {
                                                       Some(mapping(i,
                                                                    &e1.Current(),
                                                                    &e2.Current()))
                                                   } else { None::<U> }
                                               }
                                       }),
                             &Rc::from(move
                                           |tupledArg_1:
                                                &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                                  Rc<dyn Interfaces::IEnumerator_1<T2>>)|
                                           {
                                               let try_result =
                                                   tupledArg_1.0.clone().Dispose();
                                               tupledArg_1.1.clone().Dispose();
                                               try_result
                                           }))
    }
    pub fn map3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static,
                U: Clone +
                'static>(mapping:
                             &Rc<impl Fn(&T1, &T2, &T3) -> (U) + 'static>,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>,
                         zs: &Rc<dyn Interfaces::IEnumerable_1<T3>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    let ys = ys.clone();
                                    let zs = zs.clone();
                                    move ||
                                        (Seq::ofSeq(&xs), Seq::ofSeq(&ys),
                                         Seq::ofSeq(&zs))
                                }),
                      &Rc::from({
                                    let mapping = mapping.clone();
                                    move
                                        |tupledArg:
                                             &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                               Rc<dyn Interfaces::IEnumerator_1<T2>>,
                                               Rc<dyn Interfaces::IEnumerator_1<T3>>)|
                                        {
                                            let e1:
                                                    Rc<dyn Interfaces::IEnumerator_1<T1>> =
                                                tupledArg.0.clone();
                                            let e2:
                                                    Rc<dyn Interfaces::IEnumerator_1<T2>> =
                                                tupledArg.1.clone();
                                            let e3:
                                                    Rc<dyn Interfaces::IEnumerator_1<T3>> =
                                                tupledArg.2.clone();
                                            if if if e1.MoveNext() {
                                                      e2.MoveNext()
                                                  } else { false } {
                                                   e3.MoveNext()
                                               } else { false } {
                                                Some(mapping(&e1.Current(),
                                                             &e2.Current(),
                                                             &e3.Current()))
                                            } else { None::<U> }
                                        }
                                }),
                      &Rc::from(move
                                    |tupledArg_1:
                                         &(Rc<dyn Interfaces::IEnumerator_1<T1>>,
                                           Rc<dyn Interfaces::IEnumerator_1<T2>>,
                                           Rc<dyn Interfaces::IEnumerator_1<T3>>)|
                                    {
                                        let try_result =
                                            tupledArg_1.0.clone().Dispose();
                                        {
                                            let try_result_1 =
                                                tupledArg_1.1.clone().Dispose();
                                            tupledArg_1.2.clone().Dispose();
                                            try_result_1
                                        }
                                        try_result
                                    }))
    }
    pub fn readOnly<T: Clone +
                    'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::map(&Rc::from(move |x: &T| x.clone()), xs)
    }
    pub fn mapFold<State: Clone + 'static, T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&State, &T) -> ((U, State)) +
                                    'static>, state: &State,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> (Rc<dyn Interfaces::IEnumerable_1<U>>, State) {
        let patternInput: (Rc<MutCell<Vec<U>>>, State) =
            Array_::mapFold(mapping, state, &Seq::toArray(xs));
        (Seq::readOnly(&Seq::ofArray(&patternInput.0.clone())),
         patternInput.1.clone())
    }
    pub fn mapFoldBack<T: Clone + 'static, State: Clone + 'static, U: Clone +
                       'static>(mapping:
                                    &Rc<impl Fn(&T, &State) -> ((U, State)) +
                                        'static>,
                                xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                                state: &State)
     -> (Rc<dyn Interfaces::IEnumerable_1<U>>, State) {
        let patternInput: (Rc<MutCell<Vec<U>>>, State) =
            Array_::mapFoldBack(mapping, &Seq::toArray(xs), state);
        (Seq::readOnly(&Seq::ofArray(&patternInput.0.clone())),
         patternInput.1.clone())
    }
    pub fn collect<T: Clone + 'static, U: Clone +
                   'static>(mapping:
                                &Rc<impl Fn(&T)
                                    ->
                                        (Rc<dyn Interfaces::IEnumerable_1<U>>) +
                                    'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<U>> {
        Seq::delay(&Rc::from({
                                 let mapping = mapping.clone();
                                 let xs = xs.clone();
                                 move || Seq::concat(&Seq::map(&mapping, &xs))
                             }))
    }
    pub fn cache<T: Clone +
                 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        let prefix: Rc<MutCell<Vec<T>>> = Native::arrayEmpty::<T>();
        let enumOpt:
                Rc<MutCell<Option<Rc<dyn Interfaces::IEnumerator_1<T>>>>> =
            Rc::from(MutCell::from(None::<Rc<dyn Interfaces::IEnumerator_1<T>>>));
        let finished: Rc<MutCell<bool>> = Rc::from(MutCell::from(false));
        Seq::unfold(&Rc::from({
                                  let enumOpt = enumOpt.clone();
                                  let finished = finished.clone();
                                  let prefix = prefix.clone();
                                  let xs = xs.clone();
                                  move |i: &i32|
                                      if i.clone() < prefix.len() as i32 {
                                          Some((prefix[i.clone()].clone(),
                                                i.clone() + 1i32))
                                      } else {
                                          if enumOpt.get().is_none() {
                                              enumOpt.set(Some(xs.GetEnumerator()));
                                          }
                                          if enumOpt.get().is_some() {
                                              if {
                                                     let e:
                                                             Rc<dyn Interfaces::IEnumerator_1<T>> =
                                                         Option_::getValue(&enumOpt.get()).clone();
                                                     !finished.get()
                                                 } {
                                                  let e_1:
                                                          Rc<dyn Interfaces::IEnumerator_1<T>> =
                                                      Option_::getValue(&enumOpt.get()).clone();
                                                  if e_1.MoveNext() {
                                                      prefix.get_mut().push(e_1.Current().clone());
                                                      Some((e_1.Current().clone(),
                                                            i.clone() + 1i32))
                                                  } else {
                                                      finished.set(true);
                                                      None::<(T, i32)>
                                                  }
                                              } else { None::<(T, i32)> }
                                          } else { None::<(T, i32)> }
                                      }
                              }), &0i32)
    }
    pub fn allPairs<T1: Clone + 'static, T2: Clone +
                    'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                             ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(T1, T2)>> {
        let ysCache: Rc<dyn Interfaces::IEnumerable_1<T2>> = Seq::cache(ys);
        Seq::delay(&Rc::from({
                                 let xs = xs.clone();
                                 let ysCache = ysCache.clone();
                                 move ||
                                     Seq::concat(&Seq::map(&Rc::from({
                                                                         let ysCache
                                                                             =
                                                                             ysCache.clone();
                                                                         move
                                                                             |x:
                                                                                  &T1|
                                                                             Seq::map(&Rc::from({
                                                                                                    let x
                                                                                                        =
                                                                                                        x.clone();
                                                                                                    move
                                                                                                        |y:
                                                                                                             &T2|
                                                                                                        (x.clone(),
                                                                                                         y.clone())
                                                                                                }),
                                                                                      &ysCache)
                                                                     }), &xs))
                             }))
    }
    pub fn tryPick<T: Clone + 'static, a_: Clone +
                   'static>(chooser:
                                &Rc<impl Fn(&T) -> (Option<a_>) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Option<a_> {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                {
                    let res: Rc<MutCell<Option<a_>>> =
                        Rc::from(MutCell::from(None::<a_>));
                    while if res.get().is_none() {
                              e.MoveNext()
                          } else { false } {
                        res.set(chooser(&e.Current()));
                    }
                    res.get().clone()
                };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn pick<T: Clone + 'static, a_: Clone +
                'static>(chooser: &Rc<impl Fn(&T) -> (Option<a_>) + 'static>,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> a_ {
        let matchValue: Option<a_> = Seq::tryPick(chooser, xs);
        match &matchValue {
            None => panic!("{}", Seq::SR::keyNotFoundAlt()),
            Some(matchValue_0_0) => matchValue_0_0.clone(),
        }
    }
    pub fn reduce<T: Clone +
                  'static>(folder: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        let e: Rc<dyn Interfaces::IEnumerator_1<T>> = Seq::ofSeq(xs);
        {
            let try_result =
                if e.MoveNext() {
                    let acc: Rc<MutCell<T>> =
                        Rc::from(MutCell::from(e.Current().clone()));
                    while e.MoveNext() {
                        acc.set(folder(&acc.get(), &e.Current()));
                    }
                    acc.get().clone()
                } else { panic!("{}", Seq::SR::inputSequenceEmpty()) };
            if false { e.Dispose(); }
            try_result
        }
    }
    pub fn reduceBack<T: Clone +
                      'static>(folder: &Rc<impl Fn(&T, &T) -> (T) + 'static>,
                               xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> T {
        let arr: Rc<MutCell<Vec<T>>> = Seq::toArray(xs);
        if arr.len() as i32 > 0i32 {
            Array_::reduceBack(folder, &arr)
        } else { panic!("{}", Seq::SR::inputSequenceEmpty()) }
    }
    pub fn replicate<a_: Clone + 'static>(n: &i32, x: &a_)
     -> Rc<dyn Interfaces::IEnumerable_1<a_>> {
        Seq::initialize(n,
                        &Rc::from({
                                      let x = x.clone();
                                      move |_arg1: &i32| x.clone()
                                  }))
    }
    pub fn reverse<T: Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::reverse(&Seq::toArray(&xs)))
                             }))
    }
    pub fn scan<State: Clone + 'static, T: Clone +
                'static>(folder:
                             &Rc<impl Fn(&State, &T) -> (State) + 'static>,
                         state: &State,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<State>> {
        Seq::delay(&Rc::from({
                                 let folder = folder.clone();
                                 let state = state.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let acc: Rc<MutCell<State>> =
                                             Rc::from(MutCell::from(state.clone()));
                                         Seq::append(&Seq::singleton(&state),
                                                     &Seq::map(&Rc::from({
                                                                             let acc
                                                                                 =
                                                                                 acc.clone();
                                                                             let folder
                                                                                 =
                                                                                 folder.clone();
                                                                             move
                                                                                 |x:
                                                                                      &T|
                                                                                 {
                                                                                     acc.set(folder(&acc.get(),
                                                                                                    x));
                                                                                     acc.get().clone()
                                                                                 }
                                                                         }),
                                                               &xs))
                                     }
                             }))
    }
    pub fn scanBack<T: Clone + 'static, State: Clone +
                    'static>(folder:
                                 &Rc<impl Fn(&T, &State) -> (State) +
                                     'static>,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>,
                             state: &State)
     -> Rc<dyn Interfaces::IEnumerable_1<State>> {
        Seq::delay(&Rc::from({
                                 let folder = folder.clone();
                                 let state = state.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::scanBack(&folder,
                                                                    &Seq::toArray(&xs),
                                                                    &state))
                             }))
    }
    pub fn skip<T: Clone +
                'static>(count: &i32,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::mkSeq(&Rc::from({
                                 let count = count.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let e:
                                                 Rc<dyn Interfaces::IEnumerator_1<T>> =
                                             Seq::ofSeq(&xs);
                                         for i in 1i32..=count {
                                             if !e.MoveNext() {
                                                 panic!("{}",
                                                        Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
                                             };
                                         }
                                         e.clone()
                                     }
                             }))
    }
    pub fn skipWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let predicate = predicate.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let skipped: Rc<MutCell<bool>> =
                                             Rc::from(MutCell::from(true));
                                         Seq::filter(&Rc::from({
                                                                   let predicate
                                                                       =
                                                                       predicate.clone();
                                                                   let skipped
                                                                       =
                                                                       skipped.clone();
                                                                   move
                                                                       |x: &T|
                                                                       {
                                                                           if skipped.get()
                                                                              {
                                                                               skipped.set(predicate(x));
                                                                           }
                                                                           !skipped.get()
                                                                       }
                                                               }), &xs)
                                     }
                             }))
    }
    pub fn tail<T: Clone + 'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::skip(&1i32, xs)
    }
    pub fn take<T: Clone +
                'static>(count: &i32,
                         xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let count = count.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                               if i.clone() < count {
                                                   if !e.MoveNext() {
                                                       panic!("{}",
                                                              Rc::from((Rc::from(Seq::SR::notEnoughElements().to_string() +
                       &String_::string(&"\\nParameter name: ")) as
              Rc<str>).to_string() + &String_::string(&"source")) as Rc<str>);
                                                   }
                                                   Some(e.Current().clone())
                                               } else { None::<T> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                           e_1.Dispose()))
    }
    pub fn takeWhile<T: Clone +
                     'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::generate(&Rc::from({
                                    let xs = xs.clone();
                                    move || Seq::ofSeq(&xs)
                                }),
                      &Rc::from({
                                    let predicate = predicate.clone();
                                    move
                                        |e:
                                             &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                        if if e.MoveNext() {
                                               predicate(&e.Current())
                                           } else { false } {
                                            Some(e.Current().clone())
                                        } else { None::<T> }
                                }),
                      &Rc::from(move
                                    |e_1:
                                         &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                    e_1.Dispose()))
    }
    pub fn truncate<T: Clone +
                    'static>(count: &i32,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::generateIndexed(&Rc::from({
                                           let xs = xs.clone();
                                           move || Seq::ofSeq(&xs)
                                       }),
                             &Rc::from({
                                           let count = count.clone();
                                           move
                                               |i: &i32,
                                                e:
                                                    &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                               if if i.clone() < count {
                                                      e.MoveNext()
                                                  } else { false } {
                                                   Some(e.Current().clone())
                                               } else { None::<T> }
                                       }),
                             &Rc::from(move
                                           |e_1:
                                                &Rc<dyn Interfaces::IEnumerator_1<T>>|
                                           e_1.Dispose()))
    }
    pub fn zip<T1: Clone + 'static, T2: Clone +
               'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                        ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(T1, T2)>> {
        Seq::map2(&Rc::from(move |x: &T1, y: &T2| (x.clone(), y.clone())), xs,
                  ys)
    }
    pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone +
                'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T1>>,
                         ys: &Rc<dyn Interfaces::IEnumerable_1<T2>>,
                         zs: &Rc<dyn Interfaces::IEnumerable_1<T3>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(T1, T2, T3)>> {
        Seq::map3(&Rc::from(move |x: &T1, y: &T2, z: &T3|
                                (x.clone(), y.clone(), z.clone())), xs, ys,
                  zs)
    }
    pub fn pairwise<T: Clone +
                    'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<(T, T)>> {
        Seq::delay(&Rc::from({
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::pairwise(&Seq::toArray(&xs)))
                             }))
    }
    pub fn splitInto<T: Clone +
                     'static>(chunks: &i32,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let chunks = chunks.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::splitInto(&chunks,
                                                                     &Seq::toArray(&xs)))
                             }))
    }
    pub fn r#where<T: Clone +
                   'static>(predicate: &Rc<impl Fn(&T) -> (bool) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::filter(predicate, xs)
    }
    pub fn windowed<T: Clone +
                    'static>(windowSize: &i32,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let windowSize = windowSize.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::windowed(&windowSize,
                                                                    &Seq::toArray(&xs)))
                             }))
    }
    pub fn sortWith<T: Clone +
                    'static>(comparer:
                                 &Rc<impl Fn(&T, &T) -> (i32) + 'static>,
                             xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let comparer = comparer.clone();
                                 let xs = xs.clone();
                                 move ||
                                     {
                                         let arr: Rc<MutCell<Vec<T>>> =
                                             Seq::toArray(&xs);
                                         Array_::sortInPlaceWith(&comparer,
                                                                 &arr);
                                         Seq::ofArray(&arr)
                                     }
                             }))
    }
    pub fn sort<T: PartialOrd + Clone +
                'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from(move |e1: &T, e2: &T| Util::compare(e1, e2)),
                      xs)
    }
    pub fn sortBy<T: Clone + 'static, U: PartialOrd + Clone +
                  'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                           xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        Util::compare(&projection(x),
                                                      &projection(y))
                                }), xs)
    }
    pub fn sortDescending<T: PartialOrd + Clone +
                          'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from(move |x: &T, y: &T|
                                    Util::compare(x, y) * -1i32), xs)
    }
    pub fn sortByDescending<T: Clone + 'static, U: PartialOrd + Clone +
                            'static>(projection:
                                         &Rc<impl Fn(&T) -> (U) + 'static>,
                                     xs:
                                         &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::sortWith(&Rc::from({
                                    let projection = projection.clone();
                                    move |x: &T, y: &T|
                                        Util::compare(&projection(x),
                                                      &projection(y)) * -1i32
                                }), xs)
    }
    pub fn sum<T: core::ops::Add<Output = T> + Default + Clone +
               'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        Seq::fold(&Rc::from(move |acc: &T, x: &T| acc.clone() + x.clone()),
                  &Native::getZero::<T>(), xs)
    }
    pub fn sumBy<T: Clone + 'static, U: core::ops::Add<Output = U> + Default +
                 Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> U {
        Seq::fold(&Rc::from({
                                let projection = projection.clone();
                                move |acc: &U, x: &T|
                                    acc.clone() + projection(x)
                            }), &Native::getZero::<U>(), xs)
    }
    pub fn maxBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from({
                                  let projection = projection.clone();
                                  move |x: &T, y: &T|
                                      if projection(x) > projection(y) {
                                          x.clone()
                                      } else { y.clone() }
                              }), xs)
    }
    pub fn max<T: PartialOrd + Clone +
               'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from(move |x: &T, y: &T|
                                  if x.clone() > y.clone() {
                                      x.clone()
                                  } else { y.clone() }), xs)
    }
    pub fn minBy<T: Clone + 'static, U: PartialOrd + Clone +
                 'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                          xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from({
                                  let projection = projection.clone();
                                  move |x: &T, y: &T|
                                      if projection(x) < projection(y) {
                                          x.clone()
                                      } else { y.clone() }
                              }), xs)
    }
    pub fn min<T: PartialOrd + Clone +
               'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        Seq::reduce(&Rc::from(move |x: &T, y: &T|
                                  if x.clone() < y.clone() {
                                      x.clone()
                                  } else { y.clone() }), xs)
    }
    pub fn average<T: core::ops::Add<Output = T> +
                   core::ops::Div<Output = T> + From<i32> + Default + Clone +
                   'static>(xs: &Rc<dyn Interfaces::IEnumerable_1<T>>) -> T {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let total: T =
            Seq::fold(&Rc::from({
                                    let count = count.clone();
                                    move |acc: &T, x: &T|
                                        {
                                            count.set(count.get() + 1i32);
                                            acc.clone() + x.clone()
                                        }
                                }), &Native::getZero::<T>(), xs);
        if count.get() == 0i32 {
            panic!("{}", Seq::SR::inputSequenceEmpty());
        }
        total / T::from(count.get())
    }
    pub fn averageBy<T: Clone + 'static, U: core::ops::Add<Output = U> +
                     core::ops::Div<Output = U> + From<i32> + Default +
                     Clone +
                     'static>(projection: &Rc<impl Fn(&T) -> (U) + 'static>,
                              xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> U {
        let count: Rc<MutCell<i32>> = Rc::from(MutCell::from(0i32));
        let total: U =
            Seq::fold(&Rc::from({
                                    let count = count.clone();
                                    let projection = projection.clone();
                                    move |acc: &U, x: &T|
                                        {
                                            count.set(count.get() + 1i32);
                                            acc.clone() + projection(x)
                                        }
                                }), &Native::getZero::<U>(), xs);
        if count.get() == 0i32 {
            panic!("{}", Seq::SR::inputSequenceEmpty());
        }
        total / U::from(count.get())
    }
    pub fn permute<T: Clone +
                   'static>(f: &Rc<impl Fn(&i32) -> (i32) + 'static>,
                            xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<T>> {
        Seq::delay(&Rc::from({
                                 let f = f.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::permute(&f,
                                                                   &Seq::toArray(&xs)))
                             }))
    }
    pub fn chunkBySize<T: Clone +
                       'static>(chunkSize: &i32,
                                xs: &Rc<dyn Interfaces::IEnumerable_1<T>>)
     -> Rc<dyn Interfaces::IEnumerable_1<Rc<MutCell<Vec<T>>>>> {
        Seq::delay(&Rc::from({
                                 let chunkSize = chunkSize.clone();
                                 let xs = xs.clone();
                                 move ||
                                     Seq::ofArray(&Array_::chunkBySize(&chunkSize,
                                                                       &Seq::toArray(&xs)))
                             }))
    }
}
