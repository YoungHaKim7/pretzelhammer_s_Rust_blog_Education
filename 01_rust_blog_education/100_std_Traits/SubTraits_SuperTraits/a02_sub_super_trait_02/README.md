# Result

```
in super
in sub
in super
in super
in sub
in sub
in super
in sub
in sub
in super
```


```bash
(impl Supertrait)in super(CallSuperFromSub impl)
(impl Subtrait) in sub(CallSuperFromSub impl)
(impl Supertrait)in super(CallSuperFromSub impl)

(CallSubFromSuper) in super --> Supertrait for CallSubFromSuper(impl super_method
(CallSubFromSuper) in sub --> Subtrait for CallSubFromSuper(impl sub_method
(CallSubFromSuper) in sub --> Subtrait for CallSubFromSuper(impl sub_method

in super -- > Supertrait for CallEachOther(impl super_ method
in sub --> Subtrait for CallEachOther(impl sub_method
in sub --> Subtrait for CallEachOther(impl sub_method
in super -- > Supertrait for CallEachOther(impl super_ method

```
