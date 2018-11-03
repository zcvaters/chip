/* automatically generated by rust-bindgen */

use ffi_types::{mat3, vec3};

pub type std_integral_constant_value_type<_Ty> = _Ty;
pub type std_integral_constant_type = u8;
pub type std_true_type = u8;
pub type std_conditional_type<_Ty2> = _Ty2;
pub type std_conditional_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_initializer_list<_Elem> {
    pub _First: *const _Elem,
    pub _Last: *const _Elem,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Elem>>,
}
pub type std_initializer_list_value_type<_Elem> = _Elem;
pub type std_initializer_list_reference<_Elem> = *const _Elem;
pub type std_initializer_list_const_reference<_Elem> = *const _Elem;
pub type std_initializer_list_size_type = usize;
pub type std_initializer_list_iterator<_Elem> = *const _Elem;
pub type std_initializer_list_const_iterator<_Elem> = *const _Elem;
impl<_Elem> Default for std_initializer_list<_Elem> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std__Container_base0 {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}?_Orphan_all@_Container_base0@std@@QEAAXXZ"]
    pub fn std__Container_base0__Orphan_all(this: *mut std__Container_base0);
}
extern "C" {
    #[link_name = "\u{1}?_Swap_all@_Container_base0@std@@QEAAXAEAU12@@Z"]
    pub fn std__Container_base0__Swap_all(
        this: *mut std__Container_base0,
        arg1: *mut std__Container_base0,
    );
}
impl std__Container_base0 {
    #[inline]
    pub unsafe fn _Orphan_all(&mut self) {
        std__Container_base0__Orphan_all(self)
    }
    #[inline]
    pub unsafe fn _Swap_all(&mut self, arg1: *mut std__Container_base0) {
        std__Container_base0__Swap_all(self, arg1)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std__Iterator_base0 {
    pub _address: u8,
}
pub const std__Iterator_base0__Unwrap_when_unverified: bool = true;
extern "C" {
    #[link_name = "\u{1}?_Adopt@_Iterator_base0@std@@QEAAXPEBX@Z"]
    pub fn std__Iterator_base0__Adopt(
        this: *mut std__Iterator_base0,
        arg1: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}?_Getcont@_Iterator_base0@std@@QEBAPEBU_Container_base0@2@XZ"]
    pub fn std__Iterator_base0__Getcont(
        this: *const std__Iterator_base0,
    ) -> *const std__Container_base0;
}
impl std__Iterator_base0 {
    #[inline]
    pub unsafe fn _Adopt(&mut self, arg1: *const ::std::os::raw::c_void) {
        std__Iterator_base0__Adopt(self, arg1)
    }
    #[inline]
    pub unsafe fn _Getcont(&self) -> *const std__Container_base0 {
        std__Iterator_base0__Getcont(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Container_proxy {
    pub _Mycont: *const std__Container_base12,
    pub _Myfirstiter: *mut std__Iterator_base12,
}
extern "C" {
    #[link_name = "\u{1}??0_Container_proxy@std@@QEAA@XZ"]
    pub fn std__Container_proxy__Container_proxy(this: *mut std__Container_proxy);
}
impl Default for std__Container_proxy {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl std__Container_proxy {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        std__Container_proxy__Container_proxy(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Container_base12 {
    pub _Myproxy: *mut std__Container_proxy,
}
extern "C" {
    #[link_name = "\u{1}?_Getpfirst@_Container_base12@std@@QEBAPEAPEAU_Iterator_base12@2@XZ"]
    pub fn std__Container_base12__Getpfirst(
        this: *const std__Container_base12,
    ) -> *mut *mut std__Iterator_base12;
}
extern "C" {
    #[link_name = "\u{1}?_Orphan_all@_Container_base12@std@@QEAAXXZ"]
    pub fn std__Container_base12__Orphan_all(this: *mut std__Container_base12);
}
extern "C" {
    #[link_name = "\u{1}?_Swap_all@_Container_base12@std@@QEAAXAEAU12@@Z"]
    pub fn std__Container_base12__Swap_all(
        this: *mut std__Container_base12,
        arg1: *mut std__Container_base12,
    );
}
extern "C" {
    #[link_name = "\u{1}??0_Container_base12@std@@QEAA@XZ"]
    pub fn std__Container_base12__Container_base12(this: *mut std__Container_base12);
}
extern "C" {
    #[link_name = "\u{1}??0_Container_base12@std@@QEAA@AEBU01@@Z"]
    pub fn std__Container_base12__Container_base121(
        this: *mut std__Container_base12,
        arg1: *const std__Container_base12,
    );
}
impl Default for std__Container_base12 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl std__Container_base12 {
    #[inline]
    pub unsafe fn _Getpfirst(&self) -> *mut *mut std__Iterator_base12 {
        std__Container_base12__Getpfirst(self)
    }
    #[inline]
    pub unsafe fn _Orphan_all(&mut self) {
        std__Container_base12__Orphan_all(self)
    }
    #[inline]
    pub unsafe fn _Swap_all(&mut self, arg1: *mut std__Container_base12) {
        std__Container_base12__Swap_all(self, arg1)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        std__Container_base12__Container_base12(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const std__Container_base12) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        std__Container_base12__Container_base121(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct std__Iterator_base12 {
    pub _Myproxy: *mut std__Container_proxy,
    pub _Mynextiter: *mut std__Iterator_base12,
}
pub const std__Iterator_base12__Unwrap_when_unverified: bool = true;
extern "C" {
    #[link_name = "\u{1}?_Adopt@_Iterator_base12@std@@QEAAXPEBU_Container_base12@2@@Z"]
    pub fn std__Iterator_base12__Adopt(
        this: *mut std__Iterator_base12,
        _Parent: *const std__Container_base12,
    );
}
extern "C" {
    #[link_name = "\u{1}?_Clrcont@_Iterator_base12@std@@QEAAXXZ"]
    pub fn std__Iterator_base12__Clrcont(this: *mut std__Iterator_base12);
}
extern "C" {
    #[link_name = "\u{1}?_Getcont@_Iterator_base12@std@@QEBAPEBU_Container_base12@2@XZ"]
    pub fn std__Iterator_base12__Getcont(
        this: *const std__Iterator_base12,
    ) -> *const std__Container_base12;
}
extern "C" {
    #[link_name = "\u{1}?_Getpnext@_Iterator_base12@std@@QEAAPEAPEAU12@XZ"]
    pub fn std__Iterator_base12__Getpnext(
        this: *mut std__Iterator_base12,
    ) -> *mut *mut std__Iterator_base12;
}
extern "C" {
    #[link_name = "\u{1}?_Orphan_me@_Iterator_base12@std@@QEAAXXZ"]
    pub fn std__Iterator_base12__Orphan_me(this: *mut std__Iterator_base12);
}
extern "C" {
    #[link_name = "\u{1}??0_Iterator_base12@std@@QEAA@XZ"]
    pub fn std__Iterator_base12__Iterator_base12(this: *mut std__Iterator_base12);
}
extern "C" {
    #[link_name = "\u{1}??0_Iterator_base12@std@@QEAA@AEBU01@@Z"]
    pub fn std__Iterator_base12__Iterator_base121(
        this: *mut std__Iterator_base12,
        _Right: *const std__Iterator_base12,
    );
}
extern "C" {
    #[link_name = "\u{1}??_D_Iterator_base12@std@@QEAAXXZ"]
    pub fn std__Iterator_base12__Iterator_base12_destructor(this: *mut std__Iterator_base12);
}
impl Default for std__Iterator_base12 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl std__Iterator_base12 {
    #[inline]
    pub unsafe fn _Adopt(&mut self, _Parent: *const std__Container_base12) {
        std__Iterator_base12__Adopt(self, _Parent)
    }
    #[inline]
    pub unsafe fn _Clrcont(&mut self) {
        std__Iterator_base12__Clrcont(self)
    }
    #[inline]
    pub unsafe fn _Getcont(&self) -> *const std__Container_base12 {
        std__Iterator_base12__Getcont(self)
    }
    #[inline]
    pub unsafe fn _Getpnext(&mut self) -> *mut *mut std__Iterator_base12 {
        std__Iterator_base12__Getpnext(self)
    }
    #[inline]
    pub unsafe fn _Orphan_me(&mut self) {
        std__Iterator_base12__Orphan_me(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        std__Iterator_base12__Iterator_base12(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(_Right: *const std__Iterator_base12) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        std__Iterator_base12__Iterator_base121(&mut __bindgen_tmp, _Right);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        std__Iterator_base12__Iterator_base12_destructor(self)
    }
}
pub type std__Container_base = std__Container_base0;
pub type std__Iterator_base = std__Iterator_base0;
pub type std__Compressed_pair__Mybase<_Ty1> = _Ty1;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_input_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_forward_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_bidirectional_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_random_access_iterator_tag {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std__Iterator_traits_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reverse_iterator<_BidIt> {
    pub current: _BidIt,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_BidIt>>,
}
pub type std_reverse_iterator_iterator_category = std_iterator_traits;
pub type std_reverse_iterator_value_type = std_iterator_traits;
pub type std_reverse_iterator_difference_type = std_iterator_traits;
pub type std_reverse_iterator_pointer = std_iterator_traits;
pub type std_reverse_iterator_reference = std_iterator_traits;
pub type std_reverse_iterator_iterator_type<_BidIt> = _BidIt;
impl<_BidIt> Default for std_reverse_iterator<_BidIt> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std__Rebind_alloc_t = std_allocator_traits;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_allocator {
    pub _address: u8,
}
pub type std_allocator__Not_user_specialized = ::std::os::raw::c_void;
pub type std_allocator_value_type<_Ty> = _Ty;
pub type std_allocator_pointer<_Ty> = *mut _Ty;
pub type std_allocator_const_pointer<_Ty> = *const _Ty;
pub type std_allocator_reference<_Ty> = *mut _Ty;
pub type std_allocator_const_reference<_Ty> = *const _Ty;
pub type std_allocator_size_type = usize;
pub type std_allocator_difference_type = isize;
pub type std_allocator_propagate_on_container_move_assignment = std_true_type;
pub type std_allocator_is_always_equal = std_true_type;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_allocator_rebind {
    pub _address: u8,
}
pub type std_allocator_rebind_other = std_allocator;
#[repr(C)]
pub struct std__Vector_const_iterator {
    pub _Ptr: std__Vector_const_iterator__Tptr,
}
pub type std__Vector_const_iterator_iterator_category = std_random_access_iterator_tag;
pub type std__Vector_const_iterator_value_type = [u8; 0usize];
pub type std__Vector_const_iterator_difference_type = [u8; 0usize];
pub type std__Vector_const_iterator_pointer = [u8; 0usize];
pub type std__Vector_const_iterator_reference = *const std__Vector_const_iterator_value_type;
pub type std__Vector_const_iterator__Tptr = [u8; 0usize];
impl Default for std__Vector_const_iterator {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct std__Vector_iterator {
    pub _base: std__Vector_const_iterator,
}
pub type std__Vector_iterator__Mybase = std__Vector_const_iterator;
pub type std__Vector_iterator_iterator_category = std_random_access_iterator_tag;
pub type std__Vector_iterator_value_type = [u8; 0usize];
pub type std__Vector_iterator_difference_type = [u8; 0usize];
pub type std__Vector_iterator_pointer = [u8; 0usize];
pub type std__Vector_iterator_reference = *mut std__Vector_iterator_value_type;
impl Default for std__Vector_iterator {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std__Vec_base_types {
    pub _address: u8,
}
pub type std__Vec_base_types__Alty = std__Rebind_alloc_t;
pub type std__Vec_base_types__Alty_traits = std_allocator_traits;
pub type std__Vec_base_types__Val_types = std_conditional_t;
#[repr(C)]
pub struct std__Vector_val {
    pub _Myfirst: std__Vector_val_pointer,
    pub _Mylast: std__Vector_val_pointer,
    pub _Myend: std__Vector_val_pointer,
}
pub type std__Vector_val_value_type = [u8; 0usize];
pub type std__Vector_val_size_type = [u8; 0usize];
pub type std__Vector_val_difference_type = [u8; 0usize];
pub type std__Vector_val_pointer = [u8; 0usize];
pub type std__Vector_val_const_pointer = [u8; 0usize];
pub type std__Vector_val_reference = *mut std__Vector_val_value_type;
pub type std__Vector_val_const_reference = *const std__Vector_val_value_type;
impl Default for std__Vector_val {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std__Vector_alloc {
    pub _Mypair: u8,
}
pub type std__Vector_alloc__Alty = [u8; 0usize];
pub type std__Vector_alloc__Alty_traits = [u8; 0usize];
pub type std__Vector_alloc__Alproxy = std__Rebind_alloc_t;
pub type std__Vector_alloc__Alproxy_traits = std_allocator_traits;
pub type std__Vector_alloc__Val_types = [u8; 0usize];
pub type std__Vector_alloc_size_type = [u8; 0usize];
pub type std__Vector_alloc_difference_type = [u8; 0usize];
pub type std__Vector_alloc_pointer = [u8; 0usize];
pub type std__Vector_alloc_const_pointer = [u8; 0usize];
pub type std__Vector_alloc_iterator = std__Vector_iterator;
pub type std__Vector_alloc_const_iterator = std__Vector_const_iterator;
#[repr(C)]
#[derive(Debug)]
pub struct std_vector {
    pub _base: std__Vector_alloc,
}
pub type std_vector__Mybase = std__Vector_alloc;
pub type std_vector__Alty = std_vector__Mybase;
pub type std_vector__Alty_traits = std_vector__Mybase;
pub type std_vector_value_type<_Ty> = _Ty;
pub type std_vector_allocator_type<_Alloc> = _Alloc;
pub type std_vector_pointer = std_vector__Mybase;
pub type std_vector_const_pointer = std_vector__Mybase;
pub type std_vector_reference<_Ty> = *mut _Ty;
pub type std_vector_const_reference<_Ty> = *const _Ty;
pub type std_vector_size_type = std_vector__Mybase;
pub type std_vector_difference_type = std_vector__Mybase;
pub type std_vector_iterator = std_vector__Mybase;
pub type std_vector_const_iterator = std_vector__Mybase;
pub type std_vector_reverse_iterator = std_reverse_iterator<std_vector_iterator>;
pub type std_vector_const_reverse_iterator = std_reverse_iterator<std_vector_const_iterator>;
impl Default for std_vector {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct int2 {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
pub struct ray {
    pub start: vec3,
    pub direction: vec3,
}
impl Default for ray {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sphere {
    pub center: vec3,
    pub radius: f32,
}
impl Default for sphere {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct tri {
    pub p: [vec3; 3usize],
}
extern "C" {
    #[link_name = "\u{1}?center@tri@@QEBA?AV?$vec@$02@@XZ"]
    pub fn tri_center(this: *const tri) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?unit_normal@tri@@QEBA?AV?$vec@$02@@XZ"]
    pub fn tri_unit_normal(this: *const tri) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0tri@@QEAA@XZ"]
    pub fn tri_tri(this: *mut tri);
}
extern "C" {
    #[link_name = "\u{1}??0tri@@QEAA@V?$initializer_list@V?$vec@$02@@@std@@@Z"]
    pub fn tri_tri1(this: *mut tri, arg1: std_initializer_list<vec3>);
}
impl Default for tri {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl tri {
    #[inline]
    pub unsafe fn center(&self) -> vec3 {
        tri_center(self)
    }
    #[inline]
    pub unsafe fn unit_normal(&self) -> vec3 {
        tri_unit_normal(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        tri_tri(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: std_initializer_list<vec3>) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        tri_tri1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct obb {
    pub center: vec3,
    pub half_width: vec3,
    pub orientation: mat3,
}
extern "C" {
    #[link_name = "\u{1}??0obb@@QEAA@XZ"]
    pub fn obb_obb(this: *mut obb);
}
impl Default for obb {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl obb {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        obb_obb(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct aabb {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
extern "C" {
    #[link_name = "\u{1}?center@aabb@@QEBA?AV?$vec@$02@@XZ"]
    pub fn aabb_center(this: *const aabb) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@XZ"]
    pub fn aabb_aabb(this: *mut aabb);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUtri@@@Z"]
    pub fn aabb_aabb1(this: *mut aabb, arg1: *const tri);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUobb@@@Z"]
    pub fn aabb_aabb2(this: *mut aabb, arg1: *const obb);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBUsphere@@@Z"]
    pub fn aabb_aabb3(this: *mut aabb, s: *const sphere);
}
extern "C" {
    #[link_name = "\u{1}??0aabb@@QEAA@AEBU0@0@Z"]
    pub fn aabb_aabb4(this: *mut aabb, a: *const aabb, b: *const aabb);
}
impl aabb {
    #[inline]
    pub unsafe fn center(&self) -> vec3 {
        aabb_center(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb(&mut __bindgen_tmp);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: *const tri) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new2(arg1: *const obb) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb2(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new3(s: *const sphere) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb3(&mut __bindgen_tmp, s);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new4(a: *const aabb, b: *const aabb) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        aabb_aabb4(&mut __bindgen_tmp, a, b);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct bvh {
    pub global: aabb,
    pub mask: u64,
    pub num_leaves: usize,
    pub nodes: std_vector,
    pub ranges: std_vector,
    pub ready: std_vector,
    pub parents: std_vector,
    pub siblings: std_vector,
    pub code_ids: std_vector,
    pub primitives: std_vector,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bvh_bvh_node {
    pub box_: aabb,
    pub code: u64,
}
impl Default for bvh {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Pitch {
    pub empty: aabb,
    pub mesh: bvh,
    pub hits: std_vector,
    pub triangles: std_vector,
}
extern "C" {
    #[link_name = "\u{1}?in_contact_with@Pitch@@QEAA_NAEBUobb@@@Z"]
    pub fn Pitch_in_contact_with(this: *mut Pitch, o: *const obb) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?in_contact_with@Pitch@@QEAA_NAEBUsphere@@@Z"]
    pub fn Pitch_in_contact_with1(this: *mut Pitch, s: *const sphere) -> bool;
}
extern "C" {
    #[link_name = "\u{1}?last_contact_info@Pitch@@QEAA?AUray@@XZ"]
    pub fn Pitch_last_contact_info(this: *mut Pitch) -> ray;
}
extern "C" {
    #[link_name = "\u{1}?raycast_any@Pitch@@QEAA?AUray@@AEBU2@@Z"]
    pub fn Pitch_raycast_any(this: *mut Pitch, arg1: *const ray) -> ray;
}
extern "C" {
    #[link_name = "\u{1}??0Pitch@@QEAA@XZ"]
    pub fn Pitch_Pitch(this: *mut Pitch);
}
impl Default for Pitch {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Pitch {
    #[inline]
    pub unsafe fn in_contact_with(&mut self, o: *const obb) -> bool {
        Pitch_in_contact_with(self, o)
    }
    #[inline]
    pub unsafe fn in_contact_with1(&mut self, s: *const sphere) -> bool {
        Pitch_in_contact_with1(self, s)
    }
    #[inline]
    pub unsafe fn last_contact_info(&mut self) -> ray {
        Pitch_last_contact_info(self)
    }
    #[inline]
    pub unsafe fn raycast_any(&mut self, arg1: *const ray) -> ray {
        Pitch_raycast_any(self, arg1)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Pitch_Pitch(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
pub struct Ball {
    pub x: vec3,
    pub v: vec3,
    pub w: vec3,
    pub t: f32,
    pub last_bounce: ray,
    pub radius: f32,
}
extern "C" {
    #[link_name = "\u{1}?p@Ball@@2VPitch@@A"]
    pub static mut Ball_p: Pitch;
}
extern "C" {
    #[link_name = "\u{1}?collider@Ball@@QEAA?AUsphere@@XZ"]
    pub fn Ball_collider(this: *mut Ball) -> sphere;
}
extern "C" {
    #[link_name = "\u{1}?wall_nearby@Ball@@QEAA?AUray@@M@Z"]
    pub fn Ball_wall_nearby(this: *mut Ball, R: f32) -> ray;
}
extern "C" {
    #[link_name = "\u{1}?step@Ball@@QEAAXM@Z"]
    pub fn Ball_step(this: *mut Ball, dt: f32);
}
extern "C" {
    #[link_name = "\u{1}??0Ball@@QEAA@XZ"]
    pub fn Ball_Ball(this: *mut Ball);
}
impl Default for Ball {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Ball {
    #[inline]
    pub unsafe fn collider(&mut self) -> sphere {
        Ball_collider(self)
    }
    #[inline]
    pub unsafe fn wall_nearby(&mut self, R: f32) -> ray {
        Ball_wall_nearby(self, R)
    }
    #[inline]
    pub unsafe fn step(&mut self, dt: f32) {
        Ball_step(self, dt)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Ball_Ball(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Input {
    pub steer: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub throttle: f32,
    pub jump: bool,
    pub boost: bool,
    pub slide: bool,
    pub handbrake: bool,
}
#[repr(C)]
pub struct Car {
    pub x: vec3,
    pub v: vec3,
    pub w: vec3,
    pub o: mat3,
    pub supersonic: bool,
    pub jumped: bool,
    pub double_jumped: bool,
    pub on_ground: bool,
    pub boost: ::std::os::raw::c_int,
    pub can_dodge: bool,
    pub dodge_timer: f32,
    pub time: f32,
    pub hitbox_widths: vec3,
    pub hitbox_offset: vec3,
    pub team: ::std::os::raw::c_int,
    pub id: ::std::os::raw::c_int,
    pub last: Input,
}
extern "C" {
    #[link_name = "\u{1}?env@Car@@0VPitch@@A"]
    pub static mut Car_env: Pitch;
}
extern "C" {
    #[link_name = "\u{1}?step@Car@@QEAAXUInput@@M@Z"]
    pub fn Car_step(this: *mut Car, in_: Input, dt: f32);
}
extern "C" {
    #[link_name = "\u{1}?pitch_surface_normal@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_pitch_surface_normal(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?hitbox@Car@@QEAA?AUobb@@XZ"]
    pub fn Car_hitbox(this: *mut Car) -> obb;
}
extern "C" {
    #[link_name = "\u{1}?extrapolate@Car@@QEAAXM@Z"]
    pub fn Car_extrapolate(this: *mut Car, arg1: f32);
}
extern "C" {
    #[link_name = "\u{1}?forward@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_forward(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?left@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_left(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}?up@Car@@QEAA?AV?$vec@$02@@XZ"]
    pub fn Car_up(this: *mut Car) -> vec3;
}
extern "C" {
    #[link_name = "\u{1}??0Car@@QEAA@XZ"]
    pub fn Car_Car(this: *mut Car);
}
impl Default for Car {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Car {
    #[inline]
    pub unsafe fn step(&mut self, in_: Input, dt: f32) {
        Car_step(self, in_, dt)
    }
    #[inline]
    pub unsafe fn pitch_surface_normal(&mut self) -> vec3 {
        Car_pitch_surface_normal(self)
    }
    #[inline]
    pub unsafe fn hitbox(&mut self) -> obb {
        Car_hitbox(self)
    }
    #[inline]
    pub unsafe fn extrapolate(&mut self, arg1: f32) {
        Car_extrapolate(self, arg1)
    }
    #[inline]
    pub unsafe fn forward(&mut self) -> vec3 {
        Car_forward(self)
    }
    #[inline]
    pub unsafe fn left(&mut self) -> vec3 {
        Car_left(self)
    }
    #[inline]
    pub unsafe fn up(&mut self) -> vec3 {
        Car_up(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Car_Car(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}?max_curvature@@YAMM@Z"]
    pub fn max_curvature(speed: f32) -> f32;
}
extern "C" {
    #[link_name = "\u{1}?max_speed@@YAMM@Z"]
    pub fn max_speed(curvature: f32) -> f32;
}
