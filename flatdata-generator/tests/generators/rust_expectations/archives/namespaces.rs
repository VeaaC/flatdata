pub mod n {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

"#;
}

pub mod x {

pub const X: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;

pub mod resources {
pub const PAYLOAD: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;
}
}
}
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`SRef`] for the read-only access, and
/// * [`SMut`] for the mutable access
///
/// to the `S` data.
///
/// [`SRef`]: struct.SRef.html
/// [`SMut`]: struct.SMut.html
#[derive(Clone, Debug)]
pub struct S {}

/// Read-only access to [`S`].
///
/// [`S`]: struct.S.html
#[derive(Clone, Copy)]
pub struct SRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = SRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = SMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for S {}

impl<'a> SRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for SRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for SRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for SRef<'a> {}

/// Mutable access to [`S`].
///
/// [`S`]: struct.S.html
pub struct SMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &SRef) {
        self.set_x(other.x());
    }
}

impl<'a> std::fmt::Debug for SMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        SRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for SMut<'a> {}



#[derive(Clone)]
pub struct X {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    payload: flatdata::MemoryDescriptor,
}

impl X {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn payload(&self) -> flatdata::RawData {
        flatdata::RawData::new(unsafe {self.payload.as_bytes()})
    }

}

impl ::std::fmt::Debug for X {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("X")
            .field("payload", &self.payload())
            .finish()
    }
}

impl flatdata::Archive for X {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let payload = Self::read_resource(&*storage, "payload", schema::x::resources::PAYLOAD)?;

        Ok(Self {
            _storage: storage,
            payload,
        })
    }
}

/// Builder for creating [`X`] archives.
///
///[`X`]: struct.X.html
#[derive(Clone, Debug)]
pub struct XBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl XBuilder {
    /// Stores [`payload`] in the archive.
    ///
    /// [`payload`]: struct.X.html#method.payload
    #[inline]
    pub fn set_payload(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("payload", schema::x::resources::PAYLOAD, data)
    }

}

impl flatdata::ArchiveBuilder for XBuilder {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}

#[allow(missing_docs)]
pub mod m {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace m {
struct S
{
    x : u64 : 64;
}
}

"#;
}

pub mod x {

pub const X: &str = r#"namespace m {
archive X
{
    payload : raw_data;
}
}

"#;

pub mod resources {
pub const PAYLOAD: &str = r#"namespace m {
archive X
{
    payload : raw_data;
}
}

"#;
}
}
}
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`SRef`] for the read-only access, and
/// * [`SMut`] for the mutable access
///
/// to the `S` data.
///
/// [`SRef`]: struct.SRef.html
/// [`SMut`]: struct.SMut.html
#[derive(Clone, Debug)]
pub struct S {}

/// Read-only access to [`S`].
///
/// [`S`]: struct.S.html
#[derive(Clone, Copy)]
pub struct SRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = SRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = SMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for S {}

impl<'a> SRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for SRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for SRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for SRef<'a> {}

/// Mutable access to [`S`].
///
/// [`S`]: struct.S.html
pub struct SMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &SRef) {
        self.set_x(other.x());
    }
}

impl<'a> std::fmt::Debug for SMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        SRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for SMut<'a> {}



#[derive(Clone)]
pub struct X {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    payload: flatdata::MemoryDescriptor,
}

impl X {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn payload(&self) -> flatdata::RawData {
        flatdata::RawData::new(unsafe {self.payload.as_bytes()})
    }

}

impl ::std::fmt::Debug for X {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("X")
            .field("payload", &self.payload())
            .finish()
    }
}

impl flatdata::Archive for X {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let payload = Self::read_resource(&*storage, "payload", schema::x::resources::PAYLOAD)?;

        Ok(Self {
            _storage: storage,
            payload,
        })
    }
}

/// Builder for creating [`X`] archives.
///
///[`X`]: struct.X.html
#[derive(Clone, Debug)]
pub struct XBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl XBuilder {
    /// Stores [`payload`] in the archive.
    ///
    /// [`payload`]: struct.X.html#method.payload
    #[inline]
    pub fn set_payload(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("payload", schema::x::resources::PAYLOAD, data)
    }

}

impl flatdata::ArchiveBuilder for XBuilder {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}

#[allow(missing_docs)]
pub mod a {

#[doc(hidden)]
pub mod schema {
pub mod structs {
}

pub mod a {

pub const A: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    single : .n.S;
    list : vector< .m.S >;
    multi : multivector< 32, .n.S >;
    inner : archive .n.X;
}
}

"#;

pub mod resources {
pub const SINGLE: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    single : .n.S;
}
}

"#;
pub const LIST: &str = r#"namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    list : vector< .m.S >;
}
}

"#;
pub const MULTI: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    multi : multivector< 32, .n.S >;
}
}

"#;
pub const INNER: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    inner : archive .n.X;
}
}

"#;
}
}
}

/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`multi`] resource.
///
/// [`multi`]: struct.Archive{.a.A}.html#method.multi
#[derive(Clone, PartialEq)]
pub enum MultiRef<'a> {
    #[allow(missing_docs)]
    S(<super::n::S as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for MultiRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MultiRef::S(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for MultiRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            MultiRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`multi`] resource.
///
/// Refers to a single bucket in the [`multi`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`multi`]: struct.Archive{.a.A}.html#method.multi
pub struct MultiBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> MultiBuilder<'a> {
    /// Adds data of the type [`S`] to the bucket.
    ///
    /// [`S`]: struct.S.html
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

/// Variadic struct attached to the [`multi`] archive resource.
///
/// It unifies the following data types:
//
/// * [`S`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`multi`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`MultiRef`] for the read-only heterogeneous access, and
/// * [`MultiBuilder`] for the mutable builder pattern access.
///
/// [`multi`]: struct.Archive{.a.A}.html#method.multi
/// [`MultiRef`]: enum.MultiRef.html
/// [`MultiBuilder`]: struct.MultiBuilder.html
/// [`S`]: struct.S.html
#[derive(Clone)]
pub struct Multi {}

impl<'a> flatdata::VariadicStruct<'a> for Multi {
    type Index = super::_builtin::multivector::IndexType32;

    type Item = MultiRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => MultiRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type MultiRef", index),
        }
    }

    type ItemMut = MultiBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}

#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    single: flatdata::MemoryDescriptor,
    list: flatdata::MemoryDescriptor,
    multi: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
    inner: super::n::X,
}

impl A {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn single(&self) -> <super::n::S as flatdata::Struct>::Item
    {
        <super::n::S as flatdata::Struct>::create(&unsafe {self.single.as_bytes()})
    }

    #[inline]
    pub fn list(&self) -> flatdata::ArrayView<super::m::S>
    {
        flatdata::ArrayView::new(&unsafe {self.list.as_bytes()})
    }

    #[inline]
    pub fn multi(&self) -> flatdata::MultiArrayView<Multi>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.multi.0.as_bytes()}),
            &unsafe {self.multi.1.as_bytes()},
        )
    }

    #[inline]
    pub fn inner(&self) -> &super::n::X
    {
        &self.inner
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("single", &self.single())
            .field("list", &self.list())
            .field("multi", &self.multi())
            .field("inner", &self.inner())
            .finish()
    }
}

impl flatdata::Archive for A {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let single = Self::read_resource(&*storage, "single", schema::a::resources::SINGLE)?;
        let list = Self::read_resource(&*storage, "list", schema::a::resources::LIST)?;
        let multi = {
            let index_schema = &format!("index({})", schema::a::resources::MULTI);
            let index = Self::read_resource(&*storage, "multi_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "multi", schema::a::resources::MULTI)?;            (index, data)
        };
        let inner = super::n::X::open(storage.subdir("inner"))?;

        Ok(Self {
            _storage: storage,
            single,
            list,
            multi,
            inner,
        })
    }
}

/// Builder for creating [`A`] archives.
///
///[`A`]: struct.A.html
#[derive(Clone, Debug)]
pub struct ABuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl ABuilder {
    #[inline]
    /// Stores [`single`] in the archive.
    ///
    /// [`single`]: struct.A.html#method.single
    pub fn set_single(&self, resource: <super::n::S as flatdata::Struct>::Item) -> ::std::io::Result<()> {
        let data = unsafe {
            ::std::slice::from_raw_parts(resource.data, <super::n::S as flatdata::Struct>::SIZE_IN_BYTES)
        };
        self.storage.write("single", schema::a::resources::SINGLE, data)
    }

    #[inline]
    /// Stores [`list`] in the archive.
    ///
    /// [`list`]: struct.A.html#method.list
    pub fn set_list(&self, vector: &flatdata::ArrayView<super::m::S>) -> ::std::io::Result<()> {
        self.storage.write("list", schema::a::resources::LIST, vector.as_ref())
    }

    /// Opens [`list`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`list`]: struct.A.html#method.list
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_list(&self) -> ::std::io::Result<flatdata::ExternalVector<super::m::S>> {
        flatdata::create_external_vector(&*self.storage, "list", schema::a::resources::LIST)
    }

    /// Opens [`multi`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`multi`]: struct.A.html#method.multi
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_multi(&self) -> ::std::io::Result<flatdata::MultiVector<Multi>> {
        flatdata::create_multi_vector(&*self.storage, "multi", schema::a::resources::MULTI)
    }

    /// Stores [`inner`] in the archive.
    ///
    /// [`inner`]: struct.A.html#method.inner
    #[inline]
    pub fn inner(&self) -> Result<super::n::XBuilder, flatdata::ResourceStorageError> {
        use flatdata::ArchiveBuilder;
        let storage = self.storage.subdir("inner");
        super::n::XBuilder::new(storage)
    }

}

impl flatdata::ArchiveBuilder for ABuilder {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}

#[doc(hidden)]
pub mod _builtin {

#[allow(missing_docs)]
pub mod multivector {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const INDEX_TYPE32: &str = r#""#;
}

}

/// Builtin type to for MultiVector index
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`IndexType32Ref`] for the read-only access, and
/// * [`IndexType32Mut`] for the mutable access
///
/// to the `IndexType32` data.
///
/// [`IndexType32Ref`]: struct.IndexType32Ref.html
/// [`IndexType32Mut`]: struct.IndexType32Mut.html
#[derive(Clone, Debug)]
pub struct IndexType32 {}

/// Read-only access to [`IndexType32`].
///
/// [`IndexType32`]: struct.IndexType32.html
#[derive(Clone, Copy)]
pub struct IndexType32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType32
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType32Ref<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 32);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 32);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 4 * 8, 32);
        start..end
    }

}

impl<'a> std::fmt::Debug for IndexType32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType32")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType32Ref<'a> {}

/// Mutable access to [`IndexType32`].
///
/// [`IndexType32`]: struct.IndexType32.html
pub struct IndexType32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType32Mut<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType32Ref.html#method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 32);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType32Ref) {
        self.set_value(other.value());
    }
}

impl<'a> std::fmt::Debug for IndexType32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType32Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType32 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}

}

#[doc(hidden)]
pub mod schema {
pub mod structs {
}

}}
