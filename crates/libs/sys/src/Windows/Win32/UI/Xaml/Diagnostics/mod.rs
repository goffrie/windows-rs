#[cfg_attr(windows, link(name = "windows"))]
extern "cdecl" {
    #[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
    pub fn InitializeXamlDiagnostic(endpointname: ::windows_sys::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows_sys::core::PCWSTR, wsztapdllname: ::windows_sys::core::PCWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
    pub fn InitializeXamlDiagnosticsEx(endpointname: ::windows_sys::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows_sys::core::PCWSTR, wsztapdllname: ::windows_sys::core::PCWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
}
pub type IBitmapData = *mut ::core::ffi::c_void;
pub type IVisualTreeService = *mut ::core::ffi::c_void;
pub type IVisualTreeService2 = *mut ::core::ffi::c_void;
pub type IVisualTreeService3 = *mut ::core::ffi::c_void;
pub type IVisualTreeServiceCallback = *mut ::core::ffi::c_void;
pub type IVisualTreeServiceCallback2 = *mut ::core::ffi::c_void;
pub type IXamlDiagnostics = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = -2144665560i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type BaseValueSource = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceUnknown: BaseValueSource = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceDefault: BaseValueSource = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceStyle: BaseValueSource = 3i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceLocal: BaseValueSource = 4i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Inherited: BaseValueSource = 5i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const DefaultStyleTrigger: BaseValueSource = 6i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const TemplateTrigger: BaseValueSource = 7i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const StyleTrigger: BaseValueSource = 8i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ImplicitStyleReference: BaseValueSource = 9i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplate: BaseValueSource = 10i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplateTrigger: BaseValueSource = 11i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Animation: BaseValueSource = 12i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Coercion: BaseValueSource = 13i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceVisualState: BaseValueSource = 14i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
impl ::core::marker::Copy for MetadataBit {}
impl ::core::clone::Clone for MetadataBit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type RenderTargetBitmapOptions = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTarget: RenderTargetBitmapOptions = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type ResourceType = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeStatic: ResourceType = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeTheme: ResourceType = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type VisualElementState = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResolved: VisualElementState = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResourceNotFound: VisualElementState = 1i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorInvalidResource: VisualElementState = 2i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub type VisualMutationType = i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Add: VisualMutationType = 0i32;
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Remove: VisualMutationType = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for BitmapDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: ::windows_sys::core::BSTR,
    pub Value: ::windows_sys::core::BSTR,
    pub MetadataBits: i64,
}
impl ::core::marker::Copy for CollectionElementValue {}
impl ::core::clone::Clone for CollectionElementValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct EnumType {
    pub Name: ::windows_sys::core::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for EnumType {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for EnumType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
impl ::core::marker::Copy for ParentChildRelation {}
impl ::core::clone::Clone for ParentChildRelation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: ::windows_sys::core::BSTR,
    pub Name: ::windows_sys::core::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
impl ::core::marker::Copy for PropertyChainSource {}
impl ::core::clone::Clone for PropertyChainSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: ::windows_sys::core::BSTR,
    pub DeclaringType: ::windows_sys::core::BSTR,
    pub ValueType: ::windows_sys::core::BSTR,
    pub ItemType: ::windows_sys::core::BSTR,
    pub Value: ::windows_sys::core::BSTR,
    pub Overridden: super::super::super::Foundation::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: ::windows_sys::core::BSTR,
    pub PropertyChainIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct SourceInfo {
    pub FileName: ::windows_sys::core::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for SourceInfo {}
impl ::core::clone::Clone for SourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: ::windows_sys::core::BSTR,
    pub Name: ::windows_sys::core::BSTR,
    pub NumChildren: u32,
}
impl ::core::marker::Copy for VisualElement {}
impl ::core::clone::Clone for VisualElement {
    fn clone(&self) -> Self {
        *self
    }
}
