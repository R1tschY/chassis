use crate::bind::binding::Binding;
use crate::bind::linker::{LinkedBindings, Linker};
use crate::{Module};

pub struct Binder {
    bindings: Vec<Binding>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    /*    pub fn bind<T: ?Sized + 'static>(&mut self) -> BindingBuilder<T> {
        let pos = self.bind_any(RecordedBinding::untargeted(Key::for_type::<T>()));
        BindingBuilder::new(self, pos)
    }

    fn bind_any(&mut self, binding: RecordedBinding) -> usize {
        self.bindings.push(binding);
        self.bindings.len() - 1
    }*/

    pub fn use_binding(&mut self, binding: Binding) {
        self.bindings.push(binding);
    }

    /// Install a Module
    #[inline]
    pub fn install(&mut self, module: &dyn Module) {
        module.configure(self)
    }

    pub(crate) fn link(self) -> LinkedBindings {
        Linker::new(self.bindings).link()
    }
}

/*pub struct BindingBuilder<'a, T: ?Sized + 'static> {
    binder: &'a mut Binder,
    pos: usize,
    key: PhantomData<T>,
}

impl<'a, T: 'static> BindingBuilder<'a, T> {
    pub fn to_instance(&mut self, instance: T) {
        self.set_target(Arc::new(AnyFactoryImpl::new(ConstantFactory(Arc::new(
            instance,
        )))));
    }
}*/

/*impl<'a, T: ?Sized + 'static> BindingBuilder<'a, T> {
    fn new(binder: &'a mut Binder, pos: usize) -> Self {
        Self {
            binder,
            pos,
            key: PhantomData,
        }
    }

    pub fn to_arc_instance(&mut self, instance: Arc<T>) {
        self.set_target(Arc::new(AnyFactoryImpl::new(ConstantFactory(instance))));
    }

    fn set_target(&mut self, factory: AnyFactoryRef) {
        self.binder.bindings[self.pos].set_target(factory);
    }
}*/

/*pub(crate) struct RecordedBinding {
    binding: Binding,
}

impl RecordedBinding {
    pub fn untargeted(key: Key) -> Self {
        Self { key, target: None }
    }

    pub fn set_target(&mut self, factory: AnyFactoryRef) {
        self.require_untargeted();
        self.target = Some(factory);
    }

    pub fn require_untargeted(&self) {
        if self.target.is_some() {
            panic!("Implementation is set multiple times");
        }
    }
}*/
