// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgPulseAudioServerLookup1 {
    type Err;
    fn get_address(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgPulseAudioServerLookup1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get_address(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.PulseAudio.ServerLookup1", "Address")
    }
}

pub fn org_pulse_audio_server_lookup1_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgPulseAudioServerLookup1<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<'_, tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.PulseAudio.ServerLookup1", data);
    let f = ::std::sync::Arc::new(f);
    let p = factory.property::<&str, _>("Address", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(r#try!(d.get_address()));
        Ok(())
    });
    let i = i.add_p(p);
    i
}

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusIntrospectable for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = r#try!(self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        }));
        r#try!(m.as_result());
        let mut i = m.iter_init();
        let data: String = r#try!(i.read());
        Ok(data)
    }
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<'_, tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<'_, tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let data = r#try!(d.introspect());
        let rm = minfo.msg.method_return();
        let rm = rm.append1(data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg>>, Self::Err>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), Self::Err>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusProperties for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg>>, Self::Err> {
        let mut m = r#try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
        }));
        r#try!(m.as_result());
        let mut i = m.iter_init();
        let value: arg::Variant<Box<dyn arg::RefArg>> = r#try!(i.read());
        Ok(value)
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), Self::Err> {
        let mut m = r#try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Set".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
            i.append(property_name);
            i.append(value);
        }));
        r#try!(m.as_result());
        Ok(())
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>, Self::Err> {
        let mut m = r#try!(self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface_name);
        }));
        r#try!(m.as_result());
        let mut i = m.iter_init();
        let props: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> = r#try!(i.read());
        Ok(props)
    }
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusProperties<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<'_, tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<'_, tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = r#try!(i.read());
        let property_name: &str = r#try!(i.read());
        let d = fclone(minfo);
        let value = r#try!(d.get(interface_name, property_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<'_, tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = r#try!(i.read());
        let property_name: &str = r#try!(i.read());
        let value: arg::Variant<Box<dyn arg::RefArg>> = r#try!(i.read());
        let d = fclone(minfo);
        r#try!(d.set(interface_name, property_name, value));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<'_, tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = r#try!(i.read());
        let d = fclone(minfo);
        let props = r#try!(d.get_all(interface_name));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(props);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("props", "a{sv}"));
    let i = i.add_m(m);
    i
}
