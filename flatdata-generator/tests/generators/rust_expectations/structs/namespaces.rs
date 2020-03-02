///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`FooRef`] for the read-only access, and
/// * [`FooMut`] for the mutable access
///
/// to the `Foo` data.
///
/// [`FooRef`]: struct.FooRef.html
/// [`FooMut`]: struct.FooMut.html
#[derive(Clone, Debug)]
pub struct Foo {}

/// Read-only access to [`Foo`].
///
/// [`Foo`]: struct.Foo.html
#[derive(Clone, Copy)]
pub struct FooRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for Foo
{
    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = FooRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = FooMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for Foo {}

impl<'a> FooRef<'a> {
    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl<'a> std::fmt::Debug for FooRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for FooRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for FooRef<'a> {}

/// Mutable access to [`Foo`].
///
/// [`Foo`]: struct.Foo.html
pub struct FooMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> FooMut<'a> {
    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u32; value, buffer, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &FooRef) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for FooMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        FooRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for FooMut<'a> {}
}

#[allow(missing_docs)]
pub mod m {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace m {
struct Foo
{
    f : u32 : 32;
}
}

"#;
}

}
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`FooRef`] for the read-only access, and
/// * [`FooMut`] for the mutable access
///
/// to the `Foo` data.
///
/// [`FooRef`]: struct.FooRef.html
/// [`FooMut`]: struct.FooMut.html
#[derive(Clone, Debug)]
pub struct Foo {}

/// Read-only access to [`Foo`].
///
/// [`Foo`]: struct.Foo.html
#[derive(Clone, Copy)]
pub struct FooRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for Foo
{
    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = FooRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = FooMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for Foo {}

impl<'a> FooRef<'a> {
    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl<'a> std::fmt::Debug for FooRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for FooRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for FooRef<'a> {}

/// Mutable access to [`Foo`].
///
/// [`Foo`]: struct.Foo.html
pub struct FooMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> FooMut<'a> {
    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u32; value, buffer, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &FooRef) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for FooMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        FooRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for FooMut<'a> {}