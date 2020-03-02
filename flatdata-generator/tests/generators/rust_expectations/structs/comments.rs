// This is a comment about Foo
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
    const SIZE_IN_BYTES: usize = 16;
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
    // This is a comment about Foo.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    // This is a comment about Foo.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for FooRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for FooRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
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
    // This is a comment about Foo.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_a(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }

    // This is a comment about Foo.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_b(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 64, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &FooRef) {
        self.set_a(other.a());
        self.set_b(other.b());
    }
}

impl<'a> std::fmt::Debug for FooMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        FooRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for FooMut<'a> {}

/// This is a comment about Bar
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`BarRef`] for the read-only access, and
/// * [`BarMut`] for the mutable access
///
/// to the `Bar` data.
///
/// [`BarRef`]: struct.BarRef.html
/// [`BarMut`]: struct.BarMut.html
#[derive(Clone, Debug)]
pub struct Bar {}

/// Read-only access to [`Bar`].
///
/// [`Bar`]: struct.Bar.html
#[derive(Clone, Copy)]
pub struct BarRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for Bar
{
    const SCHEMA: &'static str = schema::structs::BAR;
    const SIZE_IN_BYTES: usize = 16;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = BarRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = BarMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for Bar {}

impl<'a> BarRef<'a> {
    /// This is a comment about Bar.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    /// This is a comment about Bar.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for BarRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bar")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for BarRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
}

impl<'a> flatdata::Ref for BarRef<'a> {}

/// Mutable access to [`Bar`].
///
/// [`Bar`]: struct.Bar.html
pub struct BarMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> BarMut<'a> {
    /// This is a comment about Bar.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_a(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }

    /// This is a comment about Bar.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_b(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 16)
        };
        flatdata_write_bytes!(u64; value, buffer, 64, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &BarRef) {
        self.set_a(other.a());
        self.set_b(other.b());
    }
}

impl<'a> std::fmt::Debug for BarMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        BarRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for BarMut<'a> {}