#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate libc;

pub const LIBMTP_FILETYPE_FOLDER: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_FOLDER;
pub const LIBMTP_FILETYPE_WAV: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_WAV;
pub const LIBMTP_FILETYPE_MP3: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MP3;
pub const LIBMTP_FILETYPE_WMA: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_WMA;
pub const LIBMTP_FILETYPE_OGG: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_OGG;
pub const LIBMTP_FILETYPE_AUDIBLE: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_AUDIBLE;
pub const LIBMTP_FILETYPE_MP4: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MP4;
pub const LIBMTP_FILETYPE_UNDEF_AUDIO: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_UNDEF_AUDIO;
pub const LIBMTP_FILETYPE_WMV: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_WMV;
pub const LIBMTP_FILETYPE_AVI: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_AVI;
pub const LIBMTP_FILETYPE_MPEG: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MPEG;
pub const LIBMTP_FILETYPE_ASF: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_ASF;
pub const LIBMTP_FILETYPE_QT: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_QT;
pub const LIBMTP_FILETYPE_UNDEF_VIDEO: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_UNDEF_VIDEO;
pub const LIBMTP_FILETYPE_JPEG: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_JPEG;
pub const LIBMTP_FILETYPE_JFIF: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_JFIF;
pub const LIBMTP_FILETYPE_TIFF: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_TIFF;
pub const LIBMTP_FILETYPE_BMP: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_BMP;
pub const LIBMTP_FILETYPE_GIF: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_GIF;
pub const LIBMTP_FILETYPE_PICT: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_PICT;
pub const LIBMTP_FILETYPE_PNG: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_PNG;
pub const LIBMTP_FILETYPE_VCALENDAR1: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_VCALENDAR1;
pub const LIBMTP_FILETYPE_VCALENDAR2: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_VCALENDAR2;
pub const LIBMTP_FILETYPE_VCARD2: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_VCARD2;
pub const LIBMTP_FILETYPE_VCARD3: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_VCARD3;
pub const LIBMTP_FILETYPE_WINDOWSIMAGEFORMAT: _bindgen_ty_1 =
    _bindgen_ty_1::LIBMTP_FILETYPE_WINDOWSIMAGEFORMAT;
pub const LIBMTP_FILETYPE_WINEXEC: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_WINEXEC;
pub const LIBMTP_FILETYPE_TEXT: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_TEXT;
pub const LIBMTP_FILETYPE_HTML: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_HTML;
pub const LIBMTP_FILETYPE_FIRMWARE: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_FIRMWARE;
pub const LIBMTP_FILETYPE_AAC: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_AAC;
pub const LIBMTP_FILETYPE_MEDIACARD: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MEDIACARD;
pub const LIBMTP_FILETYPE_FLAC: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_FLAC;
pub const LIBMTP_FILETYPE_MP2: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MP2;
pub const LIBMTP_FILETYPE_M4A: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_M4A;
pub const LIBMTP_FILETYPE_DOC: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_DOC;
pub const LIBMTP_FILETYPE_XML: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_XML;
pub const LIBMTP_FILETYPE_XLS: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_XLS;
pub const LIBMTP_FILETYPE_PPT: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_PPT;
pub const LIBMTP_FILETYPE_MHT: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_MHT;
pub const LIBMTP_FILETYPE_JP2: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_JP2;
pub const LIBMTP_FILETYPE_JPX: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_JPX;
pub const LIBMTP_FILETYPE_ALBUM: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_ALBUM;
pub const LIBMTP_FILETYPE_PLAYLIST: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_PLAYLIST;
pub const LIBMTP_FILETYPE_UNKNOWN: _bindgen_ty_1 = _bindgen_ty_1::LIBMTP_FILETYPE_UNKNOWN;

#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 {
    LIBMTP_FILETYPE_FOLDER = 0,
    LIBMTP_FILETYPE_WAV = 1,
    LIBMTP_FILETYPE_MP3 = 2,
    LIBMTP_FILETYPE_WMA = 3,
    LIBMTP_FILETYPE_OGG = 4,
    LIBMTP_FILETYPE_AUDIBLE = 5,
    LIBMTP_FILETYPE_MP4 = 6,
    LIBMTP_FILETYPE_UNDEF_AUDIO = 7,
    LIBMTP_FILETYPE_WMV = 8,
    LIBMTP_FILETYPE_AVI = 9,
    LIBMTP_FILETYPE_MPEG = 10,
    LIBMTP_FILETYPE_ASF = 11,
    LIBMTP_FILETYPE_QT = 12,
    LIBMTP_FILETYPE_UNDEF_VIDEO = 13,
    LIBMTP_FILETYPE_JPEG = 14,
    LIBMTP_FILETYPE_JFIF = 15,
    LIBMTP_FILETYPE_TIFF = 16,
    LIBMTP_FILETYPE_BMP = 17,
    LIBMTP_FILETYPE_GIF = 18,
    LIBMTP_FILETYPE_PICT = 19,
    LIBMTP_FILETYPE_PNG = 20,
    LIBMTP_FILETYPE_VCALENDAR1 = 21,
    LIBMTP_FILETYPE_VCALENDAR2 = 22,
    LIBMTP_FILETYPE_VCARD2 = 23,
    LIBMTP_FILETYPE_VCARD3 = 24,
    LIBMTP_FILETYPE_WINDOWSIMAGEFORMAT = 25,
    LIBMTP_FILETYPE_WINEXEC = 26,
    LIBMTP_FILETYPE_TEXT = 27,
    LIBMTP_FILETYPE_HTML = 28,
    LIBMTP_FILETYPE_FIRMWARE = 29,
    LIBMTP_FILETYPE_AAC = 30,
    LIBMTP_FILETYPE_MEDIACARD = 31,
    LIBMTP_FILETYPE_FLAC = 32,
    LIBMTP_FILETYPE_MP2 = 33,
    LIBMTP_FILETYPE_M4A = 34,
    LIBMTP_FILETYPE_DOC = 35,
    LIBMTP_FILETYPE_XML = 36,
    LIBMTP_FILETYPE_XLS = 37,
    LIBMTP_FILETYPE_PPT = 38,
    LIBMTP_FILETYPE_MHT = 39,
    LIBMTP_FILETYPE_JP2 = 40,
    LIBMTP_FILETYPE_JPX = 41,
    LIBMTP_FILETYPE_ALBUM = 42,
    LIBMTP_FILETYPE_PLAYLIST = 43,
    LIBMTP_FILETYPE_UNKNOWN = 44,
}
pub use self::_bindgen_ty_1 as LIBMTP_filetype_t;

pub const LIBMTP_PROPERTY_StorageID: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_StorageID;
pub const LIBMTP_PROPERTY_ObjectFormat: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ObjectFormat;
pub const LIBMTP_PROPERTY_ProtectionStatus: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ProtectionStatus;
pub const LIBMTP_PROPERTY_ObjectSize: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ObjectSize;
pub const LIBMTP_PROPERTY_AssociationType: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_AssociationType;
pub const LIBMTP_PROPERTY_AssociationDesc: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_AssociationDesc;
pub const LIBMTP_PROPERTY_ObjectFileName: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ObjectFileName;
pub const LIBMTP_PROPERTY_DateCreated: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DateCreated;
pub const LIBMTP_PROPERTY_DateModified: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DateModified;
pub const LIBMTP_PROPERTY_Keywords: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Keywords;
pub const LIBMTP_PROPERTY_ParentObject: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ParentObject;
pub const LIBMTP_PROPERTY_AllowedFolderContents: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_AllowedFolderContents;
pub const LIBMTP_PROPERTY_Hidden: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Hidden;
pub const LIBMTP_PROPERTY_SystemObject: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_SystemObject;
pub const LIBMTP_PROPERTY_PersistantUniqueObjectIdentifier: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PersistantUniqueObjectIdentifier;
pub const LIBMTP_PROPERTY_SyncID: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_SyncID;
pub const LIBMTP_PROPERTY_PropertyBag: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_PropertyBag;
pub const LIBMTP_PROPERTY_Name: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Name;
pub const LIBMTP_PROPERTY_CreatedBy: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_CreatedBy;
pub const LIBMTP_PROPERTY_Artist: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Artist;
pub const LIBMTP_PROPERTY_DateAuthored: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DateAuthored;
pub const LIBMTP_PROPERTY_Description: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Description;
pub const LIBMTP_PROPERTY_URLReference: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_URLReference;
pub const LIBMTP_PROPERTY_LanguageLocale: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_LanguageLocale;
pub const LIBMTP_PROPERTY_CopyrightInformation: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_CopyrightInformation;
pub const LIBMTP_PROPERTY_Source: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Source;
pub const LIBMTP_PROPERTY_OriginLocation: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_OriginLocation;
pub const LIBMTP_PROPERTY_DateAdded: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DateAdded;
pub const LIBMTP_PROPERTY_NonConsumable: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_NonConsumable;
pub const LIBMTP_PROPERTY_CorruptOrUnplayable: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_CorruptOrUnplayable;
pub const LIBMTP_PROPERTY_ProducerSerialNumber: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ProducerSerialNumber;
pub const LIBMTP_PROPERTY_RepresentativeSampleFormat: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleFormat;
pub const LIBMTP_PROPERTY_RepresentativeSampleSize: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleSize;
pub const LIBMTP_PROPERTY_RepresentativeSampleHeight: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleHeight;
pub const LIBMTP_PROPERTY_RepresentativeSampleWidth: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleWidth;
pub const LIBMTP_PROPERTY_RepresentativeSampleDuration: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleDuration;
pub const LIBMTP_PROPERTY_RepresentativeSampleData: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_RepresentativeSampleData;
pub const LIBMTP_PROPERTY_Width: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Width;
pub const LIBMTP_PROPERTY_Height: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Height;
pub const LIBMTP_PROPERTY_Duration: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Duration;
pub const LIBMTP_PROPERTY_Rating: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Rating;
pub const LIBMTP_PROPERTY_Track: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Track;
pub const LIBMTP_PROPERTY_Genre: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Genre;
pub const LIBMTP_PROPERTY_Credits: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Credits;
pub const LIBMTP_PROPERTY_Lyrics: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Lyrics;
pub const LIBMTP_PROPERTY_SubscriptionContentID: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_SubscriptionContentID;
pub const LIBMTP_PROPERTY_ProducedBy: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ProducedBy;
pub const LIBMTP_PROPERTY_UseCount: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_UseCount;
pub const LIBMTP_PROPERTY_SkipCount: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_SkipCount;
pub const LIBMTP_PROPERTY_LastAccessed: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_LastAccessed;
pub const LIBMTP_PROPERTY_ParentalRating: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ParentalRating;
pub const LIBMTP_PROPERTY_MetaGenre: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MetaGenre;
pub const LIBMTP_PROPERTY_Composer: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Composer;
pub const LIBMTP_PROPERTY_EffectiveRating: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EffectiveRating;
pub const LIBMTP_PROPERTY_Subtitle: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Subtitle;
pub const LIBMTP_PROPERTY_OriginalReleaseDate: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_OriginalReleaseDate;
pub const LIBMTP_PROPERTY_AlbumName: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_AlbumName;
pub const LIBMTP_PROPERTY_AlbumArtist: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_AlbumArtist;
pub const LIBMTP_PROPERTY_Mood: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Mood;
pub const LIBMTP_PROPERTY_DRMStatus: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DRMStatus;
pub const LIBMTP_PROPERTY_SubDescription: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_SubDescription;
pub const LIBMTP_PROPERTY_IsCropped: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_IsCropped;
pub const LIBMTP_PROPERTY_IsColorCorrected: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_IsColorCorrected;
pub const LIBMTP_PROPERTY_ImageBitDepth: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ImageBitDepth;
pub const LIBMTP_PROPERTY_Fnumber: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Fnumber;
pub const LIBMTP_PROPERTY_ExposureTime: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ExposureTime;
pub const LIBMTP_PROPERTY_ExposureIndex: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ExposureIndex;
pub const LIBMTP_PROPERTY_DisplayName: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_DisplayName;
pub const LIBMTP_PROPERTY_BodyText: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_BodyText;
pub const LIBMTP_PROPERTY_Subject: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Subject;
pub const LIBMTP_PROPERTY_Priority: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Priority;
pub const LIBMTP_PROPERTY_GivenName: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_GivenName;
pub const LIBMTP_PROPERTY_MiddleNames: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MiddleNames;
pub const LIBMTP_PROPERTY_FamilyName: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_FamilyName;
pub const LIBMTP_PROPERTY_Prefix: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Prefix;
pub const LIBMTP_PROPERTY_Suffix: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Suffix;
pub const LIBMTP_PROPERTY_PhoneticGivenName: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneticGivenName;
pub const LIBMTP_PROPERTY_PhoneticFamilyName: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneticFamilyName;
pub const LIBMTP_PROPERTY_EmailPrimary: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_EmailPrimary;
pub const LIBMTP_PROPERTY_EmailPersonal1: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EmailPersonal1;
pub const LIBMTP_PROPERTY_EmailPersonal2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EmailPersonal2;
pub const LIBMTP_PROPERTY_EmailBusiness1: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EmailBusiness1;
pub const LIBMTP_PROPERTY_EmailBusiness2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EmailBusiness2;
pub const LIBMTP_PROPERTY_EmailOthers: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_EmailOthers;
pub const LIBMTP_PROPERTY_PhoneNumberPrimary: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberPrimary;
pub const LIBMTP_PROPERTY_PhoneNumberPersonal: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberPersonal;
pub const LIBMTP_PROPERTY_PhoneNumberPersonal2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberPersonal2;
pub const LIBMTP_PROPERTY_PhoneNumberBusiness: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberBusiness;
pub const LIBMTP_PROPERTY_PhoneNumberBusiness2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberBusiness2;
pub const LIBMTP_PROPERTY_PhoneNumberMobile: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberMobile;
pub const LIBMTP_PROPERTY_PhoneNumberMobile2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberMobile2;
pub const LIBMTP_PROPERTY_FaxNumberPrimary: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_FaxNumberPrimary;
pub const LIBMTP_PROPERTY_FaxNumberPersonal: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_FaxNumberPersonal;
pub const LIBMTP_PROPERTY_FaxNumberBusiness: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_FaxNumberBusiness;
pub const LIBMTP_PROPERTY_PagerNumber: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_PagerNumber;
pub const LIBMTP_PROPERTY_PhoneNumberOthers: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneNumberOthers;
pub const LIBMTP_PROPERTY_PrimaryWebAddress: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PrimaryWebAddress;
pub const LIBMTP_PROPERTY_PersonalWebAddress: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PersonalWebAddress;
pub const LIBMTP_PROPERTY_BusinessWebAddress: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_BusinessWebAddress;
pub const LIBMTP_PROPERTY_InstantMessengerAddress: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_InstantMessengerAddress;
pub const LIBMTP_PROPERTY_InstantMessengerAddress2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_InstantMessengerAddress2;
pub const LIBMTP_PROPERTY_InstantMessengerAddress3: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_InstantMessengerAddress3;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFull: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFull;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullLine1: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullLine1;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullLine2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullLine2;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullCity: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullCity;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullRegion: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullRegion;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullPostalCode: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullPostalCode;
pub const LIBMTP_PROPERTY_PostalAddressPersonalFullCountry: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressPersonalFullCountry;
pub const LIBMTP_PROPERTY_PostalAddressBusinessFull: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessFull;
pub const LIBMTP_PROPERTY_PostalAddressBusinessLine1: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessLine1;
pub const LIBMTP_PROPERTY_PostalAddressBusinessLine2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessLine2;
pub const LIBMTP_PROPERTY_PostalAddressBusinessCity: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessCity;
pub const LIBMTP_PROPERTY_PostalAddressBusinessRegion: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessRegion;
pub const LIBMTP_PROPERTY_PostalAddressBusinessPostalCode: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessPostalCode;
pub const LIBMTP_PROPERTY_PostalAddressBusinessCountry: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressBusinessCountry;
pub const LIBMTP_PROPERTY_PostalAddressOtherFull: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherFull;
pub const LIBMTP_PROPERTY_PostalAddressOtherLine1: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherLine1;
pub const LIBMTP_PROPERTY_PostalAddressOtherLine2: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherLine2;
pub const LIBMTP_PROPERTY_PostalAddressOtherCity: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherCity;
pub const LIBMTP_PROPERTY_PostalAddressOtherRegion: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherRegion;
pub const LIBMTP_PROPERTY_PostalAddressOtherPostalCode: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherPostalCode;
pub const LIBMTP_PROPERTY_PostalAddressOtherCountry: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PostalAddressOtherCountry;
pub const LIBMTP_PROPERTY_OrganizationName: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_OrganizationName;
pub const LIBMTP_PROPERTY_PhoneticOrganizationName: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_PhoneticOrganizationName;
pub const LIBMTP_PROPERTY_Role: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Role;
pub const LIBMTP_PROPERTY_Birthdate: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Birthdate;
pub const LIBMTP_PROPERTY_MessageTo: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MessageTo;
pub const LIBMTP_PROPERTY_MessageCC: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MessageCC;
pub const LIBMTP_PROPERTY_MessageBCC: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MessageBCC;
pub const LIBMTP_PROPERTY_MessageRead: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MessageRead;
pub const LIBMTP_PROPERTY_MessageReceivedTime: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_MessageReceivedTime;
pub const LIBMTP_PROPERTY_MessageSender: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_MessageSender;
pub const LIBMTP_PROPERTY_ActivityBeginTime: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityBeginTime;
pub const LIBMTP_PROPERTY_ActivityEndTime: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityEndTime;
pub const LIBMTP_PROPERTY_ActivityLocation: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityLocation;
pub const LIBMTP_PROPERTY_ActivityRequiredAttendees: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityRequiredAttendees;
pub const LIBMTP_PROPERTY_ActivityOptionalAttendees: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityOptionalAttendees;
pub const LIBMTP_PROPERTY_ActivityResources: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityResources;
pub const LIBMTP_PROPERTY_ActivityAccepted: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ActivityAccepted;
pub const LIBMTP_PROPERTY_Owner: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Owner;
pub const LIBMTP_PROPERTY_Editor: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Editor;
pub const LIBMTP_PROPERTY_Webmaster: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_Webmaster;
pub const LIBMTP_PROPERTY_URLSource: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_URLSource;
pub const LIBMTP_PROPERTY_URLDestination: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_URLDestination;
pub const LIBMTP_PROPERTY_TimeBookmark: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_TimeBookmark;
pub const LIBMTP_PROPERTY_ObjectBookmark: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_ObjectBookmark;
pub const LIBMTP_PROPERTY_ByteBookmark: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ByteBookmark;
pub const LIBMTP_PROPERTY_LastBuildDate: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_LastBuildDate;
pub const LIBMTP_PROPERTY_TimetoLive: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_TimetoLive;
pub const LIBMTP_PROPERTY_MediaGUID: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_MediaGUID;
pub const LIBMTP_PROPERTY_TotalBitRate: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_TotalBitRate;
pub const LIBMTP_PROPERTY_BitRateType: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_BitRateType;
pub const LIBMTP_PROPERTY_SampleRate: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_SampleRate;
pub const LIBMTP_PROPERTY_NumberOfChannels: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_NumberOfChannels;
pub const LIBMTP_PROPERTY_AudioBitDepth: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_AudioBitDepth;
pub const LIBMTP_PROPERTY_ScanDepth: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_ScanDepth;
pub const LIBMTP_PROPERTY_AudioWAVECodec: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_AudioWAVECodec;
pub const LIBMTP_PROPERTY_AudioBitRate: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_AudioBitRate;
pub const LIBMTP_PROPERTY_VideoFourCCCodec: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_VideoFourCCCodec;
pub const LIBMTP_PROPERTY_VideoBitRate: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_VideoBitRate;
pub const LIBMTP_PROPERTY_FramesPerThousandSeconds: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_FramesPerThousandSeconds;
pub const LIBMTP_PROPERTY_KeyFrameDistance: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_KeyFrameDistance;
pub const LIBMTP_PROPERTY_BufferSize: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_BufferSize;
pub const LIBMTP_PROPERTY_EncodingQuality: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EncodingQuality;
pub const LIBMTP_PROPERTY_EncodingProfile: _bindgen_ty_2 =
    _bindgen_ty_2::LIBMTP_PROPERTY_EncodingProfile;
pub const LIBMTP_PROPERTY_BuyFlag: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_BuyFlag;
pub const LIBMTP_PROPERTY_UNKNOWN: _bindgen_ty_2 = _bindgen_ty_2::LIBMTP_PROPERTY_UNKNOWN;

#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_2 {
    LIBMTP_PROPERTY_StorageID = 0,
    LIBMTP_PROPERTY_ObjectFormat = 1,
    LIBMTP_PROPERTY_ProtectionStatus = 2,
    LIBMTP_PROPERTY_ObjectSize = 3,
    LIBMTP_PROPERTY_AssociationType = 4,
    LIBMTP_PROPERTY_AssociationDesc = 5,
    LIBMTP_PROPERTY_ObjectFileName = 6,
    LIBMTP_PROPERTY_DateCreated = 7,
    LIBMTP_PROPERTY_DateModified = 8,
    LIBMTP_PROPERTY_Keywords = 9,
    LIBMTP_PROPERTY_ParentObject = 10,
    LIBMTP_PROPERTY_AllowedFolderContents = 11,
    LIBMTP_PROPERTY_Hidden = 12,
    LIBMTP_PROPERTY_SystemObject = 13,
    LIBMTP_PROPERTY_PersistantUniqueObjectIdentifier = 14,
    LIBMTP_PROPERTY_SyncID = 15,
    LIBMTP_PROPERTY_PropertyBag = 16,
    LIBMTP_PROPERTY_Name = 17,
    LIBMTP_PROPERTY_CreatedBy = 18,
    LIBMTP_PROPERTY_Artist = 19,
    LIBMTP_PROPERTY_DateAuthored = 20,
    LIBMTP_PROPERTY_Description = 21,
    LIBMTP_PROPERTY_URLReference = 22,
    LIBMTP_PROPERTY_LanguageLocale = 23,
    LIBMTP_PROPERTY_CopyrightInformation = 24,
    LIBMTP_PROPERTY_Source = 25,
    LIBMTP_PROPERTY_OriginLocation = 26,
    LIBMTP_PROPERTY_DateAdded = 27,
    LIBMTP_PROPERTY_NonConsumable = 28,
    LIBMTP_PROPERTY_CorruptOrUnplayable = 29,
    LIBMTP_PROPERTY_ProducerSerialNumber = 30,
    LIBMTP_PROPERTY_RepresentativeSampleFormat = 31,
    LIBMTP_PROPERTY_RepresentativeSampleSize = 32,
    LIBMTP_PROPERTY_RepresentativeSampleHeight = 33,
    LIBMTP_PROPERTY_RepresentativeSampleWidth = 34,
    LIBMTP_PROPERTY_RepresentativeSampleDuration = 35,
    LIBMTP_PROPERTY_RepresentativeSampleData = 36,
    LIBMTP_PROPERTY_Width = 37,
    LIBMTP_PROPERTY_Height = 38,
    LIBMTP_PROPERTY_Duration = 39,
    LIBMTP_PROPERTY_Rating = 40,
    LIBMTP_PROPERTY_Track = 41,
    LIBMTP_PROPERTY_Genre = 42,
    LIBMTP_PROPERTY_Credits = 43,
    LIBMTP_PROPERTY_Lyrics = 44,
    LIBMTP_PROPERTY_SubscriptionContentID = 45,
    LIBMTP_PROPERTY_ProducedBy = 46,
    LIBMTP_PROPERTY_UseCount = 47,
    LIBMTP_PROPERTY_SkipCount = 48,
    LIBMTP_PROPERTY_LastAccessed = 49,
    LIBMTP_PROPERTY_ParentalRating = 50,
    LIBMTP_PROPERTY_MetaGenre = 51,
    LIBMTP_PROPERTY_Composer = 52,
    LIBMTP_PROPERTY_EffectiveRating = 53,
    LIBMTP_PROPERTY_Subtitle = 54,
    LIBMTP_PROPERTY_OriginalReleaseDate = 55,
    LIBMTP_PROPERTY_AlbumName = 56,
    LIBMTP_PROPERTY_AlbumArtist = 57,
    LIBMTP_PROPERTY_Mood = 58,
    LIBMTP_PROPERTY_DRMStatus = 59,
    LIBMTP_PROPERTY_SubDescription = 60,
    LIBMTP_PROPERTY_IsCropped = 61,
    LIBMTP_PROPERTY_IsColorCorrected = 62,
    LIBMTP_PROPERTY_ImageBitDepth = 63,
    LIBMTP_PROPERTY_Fnumber = 64,
    LIBMTP_PROPERTY_ExposureTime = 65,
    LIBMTP_PROPERTY_ExposureIndex = 66,
    LIBMTP_PROPERTY_DisplayName = 67,
    LIBMTP_PROPERTY_BodyText = 68,
    LIBMTP_PROPERTY_Subject = 69,
    LIBMTP_PROPERTY_Priority = 70,
    LIBMTP_PROPERTY_GivenName = 71,
    LIBMTP_PROPERTY_MiddleNames = 72,
    LIBMTP_PROPERTY_FamilyName = 73,
    LIBMTP_PROPERTY_Prefix = 74,
    LIBMTP_PROPERTY_Suffix = 75,
    LIBMTP_PROPERTY_PhoneticGivenName = 76,
    LIBMTP_PROPERTY_PhoneticFamilyName = 77,
    LIBMTP_PROPERTY_EmailPrimary = 78,
    LIBMTP_PROPERTY_EmailPersonal1 = 79,
    LIBMTP_PROPERTY_EmailPersonal2 = 80,
    LIBMTP_PROPERTY_EmailBusiness1 = 81,
    LIBMTP_PROPERTY_EmailBusiness2 = 82,
    LIBMTP_PROPERTY_EmailOthers = 83,
    LIBMTP_PROPERTY_PhoneNumberPrimary = 84,
    LIBMTP_PROPERTY_PhoneNumberPersonal = 85,
    LIBMTP_PROPERTY_PhoneNumberPersonal2 = 86,
    LIBMTP_PROPERTY_PhoneNumberBusiness = 87,
    LIBMTP_PROPERTY_PhoneNumberBusiness2 = 88,
    LIBMTP_PROPERTY_PhoneNumberMobile = 89,
    LIBMTP_PROPERTY_PhoneNumberMobile2 = 90,
    LIBMTP_PROPERTY_FaxNumberPrimary = 91,
    LIBMTP_PROPERTY_FaxNumberPersonal = 92,
    LIBMTP_PROPERTY_FaxNumberBusiness = 93,
    LIBMTP_PROPERTY_PagerNumber = 94,
    LIBMTP_PROPERTY_PhoneNumberOthers = 95,
    LIBMTP_PROPERTY_PrimaryWebAddress = 96,
    LIBMTP_PROPERTY_PersonalWebAddress = 97,
    LIBMTP_PROPERTY_BusinessWebAddress = 98,
    LIBMTP_PROPERTY_InstantMessengerAddress = 99,
    LIBMTP_PROPERTY_InstantMessengerAddress2 = 100,
    LIBMTP_PROPERTY_InstantMessengerAddress3 = 101,
    LIBMTP_PROPERTY_PostalAddressPersonalFull = 102,
    LIBMTP_PROPERTY_PostalAddressPersonalFullLine1 = 103,
    LIBMTP_PROPERTY_PostalAddressPersonalFullLine2 = 104,
    LIBMTP_PROPERTY_PostalAddressPersonalFullCity = 105,
    LIBMTP_PROPERTY_PostalAddressPersonalFullRegion = 106,
    LIBMTP_PROPERTY_PostalAddressPersonalFullPostalCode = 107,
    LIBMTP_PROPERTY_PostalAddressPersonalFullCountry = 108,
    LIBMTP_PROPERTY_PostalAddressBusinessFull = 109,
    LIBMTP_PROPERTY_PostalAddressBusinessLine1 = 110,
    LIBMTP_PROPERTY_PostalAddressBusinessLine2 = 111,
    LIBMTP_PROPERTY_PostalAddressBusinessCity = 112,
    LIBMTP_PROPERTY_PostalAddressBusinessRegion = 113,
    LIBMTP_PROPERTY_PostalAddressBusinessPostalCode = 114,
    LIBMTP_PROPERTY_PostalAddressBusinessCountry = 115,
    LIBMTP_PROPERTY_PostalAddressOtherFull = 116,
    LIBMTP_PROPERTY_PostalAddressOtherLine1 = 117,
    LIBMTP_PROPERTY_PostalAddressOtherLine2 = 118,
    LIBMTP_PROPERTY_PostalAddressOtherCity = 119,
    LIBMTP_PROPERTY_PostalAddressOtherRegion = 120,
    LIBMTP_PROPERTY_PostalAddressOtherPostalCode = 121,
    LIBMTP_PROPERTY_PostalAddressOtherCountry = 122,
    LIBMTP_PROPERTY_OrganizationName = 123,
    LIBMTP_PROPERTY_PhoneticOrganizationName = 124,
    LIBMTP_PROPERTY_Role = 125,
    LIBMTP_PROPERTY_Birthdate = 126,
    LIBMTP_PROPERTY_MessageTo = 127,
    LIBMTP_PROPERTY_MessageCC = 128,
    LIBMTP_PROPERTY_MessageBCC = 129,
    LIBMTP_PROPERTY_MessageRead = 130,
    LIBMTP_PROPERTY_MessageReceivedTime = 131,
    LIBMTP_PROPERTY_MessageSender = 132,
    LIBMTP_PROPERTY_ActivityBeginTime = 133,
    LIBMTP_PROPERTY_ActivityEndTime = 134,
    LIBMTP_PROPERTY_ActivityLocation = 135,
    LIBMTP_PROPERTY_ActivityRequiredAttendees = 136,
    LIBMTP_PROPERTY_ActivityOptionalAttendees = 137,
    LIBMTP_PROPERTY_ActivityResources = 138,
    LIBMTP_PROPERTY_ActivityAccepted = 139,
    LIBMTP_PROPERTY_Owner = 140,
    LIBMTP_PROPERTY_Editor = 141,
    LIBMTP_PROPERTY_Webmaster = 142,
    LIBMTP_PROPERTY_URLSource = 143,
    LIBMTP_PROPERTY_URLDestination = 144,
    LIBMTP_PROPERTY_TimeBookmark = 145,
    LIBMTP_PROPERTY_ObjectBookmark = 146,
    LIBMTP_PROPERTY_ByteBookmark = 147,
    LIBMTP_PROPERTY_LastBuildDate = 148,
    LIBMTP_PROPERTY_TimetoLive = 149,
    LIBMTP_PROPERTY_MediaGUID = 150,
    LIBMTP_PROPERTY_TotalBitRate = 151,
    LIBMTP_PROPERTY_BitRateType = 152,
    LIBMTP_PROPERTY_SampleRate = 153,
    LIBMTP_PROPERTY_NumberOfChannels = 154,
    LIBMTP_PROPERTY_AudioBitDepth = 155,
    LIBMTP_PROPERTY_ScanDepth = 156,
    LIBMTP_PROPERTY_AudioWAVECodec = 157,
    LIBMTP_PROPERTY_AudioBitRate = 158,
    LIBMTP_PROPERTY_VideoFourCCCodec = 159,
    LIBMTP_PROPERTY_VideoBitRate = 160,
    LIBMTP_PROPERTY_FramesPerThousandSeconds = 161,
    LIBMTP_PROPERTY_KeyFrameDistance = 162,
    LIBMTP_PROPERTY_BufferSize = 163,
    LIBMTP_PROPERTY_EncodingQuality = 164,
    LIBMTP_PROPERTY_EncodingProfile = 165,
    LIBMTP_PROPERTY_BuyFlag = 166,
    LIBMTP_PROPERTY_UNKNOWN = 167,
}
pub use self::_bindgen_ty_2 as LIBMTP_property_t;

pub const LIBMTP_DATATYPE_INT8: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_INT8;
pub const LIBMTP_DATATYPE_UINT8: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_UINT8;
pub const LIBMTP_DATATYPE_INT16: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_INT16;
pub const LIBMTP_DATATYPE_UINT16: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_UINT16;
pub const LIBMTP_DATATYPE_INT32: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_INT32;
pub const LIBMTP_DATATYPE_UINT32: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_UINT32;
pub const LIBMTP_DATATYPE_INT64: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_INT64;
pub const LIBMTP_DATATYPE_UINT64: _bindgen_ty_3 = _bindgen_ty_3::LIBMTP_DATATYPE_UINT64;

#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_3 {
    LIBMTP_DATATYPE_INT8 = 0,
    LIBMTP_DATATYPE_UINT8 = 1,
    LIBMTP_DATATYPE_INT16 = 2,
    LIBMTP_DATATYPE_UINT16 = 3,
    LIBMTP_DATATYPE_INT32 = 4,
    LIBMTP_DATATYPE_UINT32 = 5,
    LIBMTP_DATATYPE_INT64 = 6,
    LIBMTP_DATATYPE_UINT64 = 7,
}
pub use self::_bindgen_ty_3 as LIBMTP_datatype_t;

pub const LIBMTP_DEVICECAP_GetPartialObject: _bindgen_ty_4 =
    _bindgen_ty_4::LIBMTP_DEVICECAP_GetPartialObject;
pub const LIBMTP_DEVICECAP_SendPartialObject: _bindgen_ty_4 =
    _bindgen_ty_4::LIBMTP_DEVICECAP_SendPartialObject;
pub const LIBMTP_DEVICECAP_EditObjects: _bindgen_ty_4 = _bindgen_ty_4::LIBMTP_DEVICECAP_EditObjects;

#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_4 {
    LIBMTP_DEVICECAP_GetPartialObject = 0,
    LIBMTP_DEVICECAP_SendPartialObject = 1,
    LIBMTP_DEVICECAP_EditObjects = 2,
}
pub use self::_bindgen_ty_4 as LIBMTP_devicecap_t;

pub const LIBMTP_ERROR_NONE: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_NONE;
pub const LIBMTP_ERROR_GENERAL: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_GENERAL;
pub const LIBMTP_ERROR_PTP_LAYER: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_PTP_LAYER;
pub const LIBMTP_ERROR_USB_LAYER: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_USB_LAYER;
pub const LIBMTP_ERROR_MEMORY_ALLOCATION: _bindgen_ty_5 =
    _bindgen_ty_5::LIBMTP_ERROR_MEMORY_ALLOCATION;
pub const LIBMTP_ERROR_NO_DEVICE_ATTACHED: _bindgen_ty_5 =
    _bindgen_ty_5::LIBMTP_ERROR_NO_DEVICE_ATTACHED;
pub const LIBMTP_ERROR_STORAGE_FULL: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_STORAGE_FULL;
pub const LIBMTP_ERROR_CONNECTING: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_CONNECTING;
pub const LIBMTP_ERROR_CANCELLED: _bindgen_ty_5 = _bindgen_ty_5::LIBMTP_ERROR_CANCELLED;

#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_5 {
    LIBMTP_ERROR_NONE = 0,
    LIBMTP_ERROR_GENERAL = 1,
    LIBMTP_ERROR_PTP_LAYER = 2,
    LIBMTP_ERROR_USB_LAYER = 3,
    LIBMTP_ERROR_MEMORY_ALLOCATION = 4,
    LIBMTP_ERROR_NO_DEVICE_ATTACHED = 5,
    LIBMTP_ERROR_STORAGE_FULL = 6,
    LIBMTP_ERROR_CONNECTING = 7,
    LIBMTP_ERROR_CANCELLED = 8,
}
pub use self::_bindgen_ty_5 as LIBMTP_error_number_t;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_device_entry_struct {
    pub vendor: *mut libc::c_char,
    pub vendor_id: u16,
    pub product: *mut libc::c_char,
    pub product_id: u16,
    pub device_flags: u32,
}

#[test]
fn bindgen_test_layout_LIBMTP_device_entry_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_device_entry_struct>(), 20usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_device_entry_struct>(), 4usize);
}

impl Clone for LIBMTP_device_entry_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_device_entry_t = LIBMTP_device_entry_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_raw_device_struct {
    pub device_entry: LIBMTP_device_entry_t,
    pub bus_location: u32,
    pub devnum: u8,
}
#[test]
fn bindgen_test_layout_LIBMTP_raw_device_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_raw_device_struct>(), 28usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_raw_device_struct>(), 4usize);
}
impl Clone for LIBMTP_raw_device_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_raw_device_t = LIBMTP_raw_device_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_error_struct {
    pub errornumber: LIBMTP_error_number_t,
    pub error_text: *mut libc::c_char,
    pub next: *mut LIBMTP_error_t,
}

#[test]
fn bindgen_test_layout_LIBMTP_error_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_error_struct>(), 12usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_error_struct>(), 4usize);
}

impl Clone for LIBMTP_error_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_error_t = LIBMTP_error_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_allowed_values_struct {
    pub u8max: u8,
    pub u8min: u8,
    pub u8step: u8,
    pub u8vals: *mut u8,
    pub i8max: i8,
    pub i8min: i8,
    pub i8step: i8,
    pub i8vals: *mut i8,
    pub u16max: u16,
    pub u16min: u16,
    pub u16step: u16,
    pub u16vals: *mut u16,
    pub i16max: i16,
    pub i16min: i16,
    pub i16step: i16,
    pub i16vals: *mut i16,
    pub u32max: u32,
    pub u32min: u32,
    pub u32step: u32,
    pub u32vals: *mut u32,
    pub i32max: i32,
    pub i32min: i32,
    pub i32step: i32,
    pub i32vals: *mut i32,
    pub u64max: u64,
    pub u64min: u64,
    pub u64step: u64,
    pub u64vals: *mut u64,
    pub i64max: i64,
    pub i64min: i64,
    pub i64step: i64,
    pub i64vals: *mut i64,
    pub num_entries: u16,
    pub datatype: LIBMTP_datatype_t,
    pub is_range: libc::c_int,
}

#[test]
fn bindgen_test_layout_LIBMTP_allowed_values_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_allowed_values_struct>(),
               140usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_allowed_values_struct>(),
               4usize);
}
impl Clone for LIBMTP_allowed_values_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_allowed_values_t = LIBMTP_allowed_values_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_device_extension_struct {
    pub name: *mut libc::c_char,
    pub major: libc::c_int,
    pub minor: libc::c_int,
    pub next: *mut LIBMTP_device_extension_t,
}

#[test]
fn bindgen_test_layout_LIBMTP_device_extension_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_device_extension_struct>(),
               16usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_device_extension_struct>(),
               4usize);
}
impl Clone for LIBMTP_device_extension_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_device_extension_t = LIBMTP_device_extension_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_mtpdevice_struct {
    pub object_bitsize: u8,
    pub params: *mut libc::c_void,
    pub usbinfo: *mut libc::c_void,
    pub storage: *mut LIBMTP_devicestorage_t,
    pub errorstack: *mut LIBMTP_error_t,
    pub maximum_battery_level: u8,
    pub default_music_folder: u32,
    pub default_playlist_folder: u32,
    pub default_picture_folder: u32,
    pub default_video_folder: u32,
    pub default_organizer_folder: u32,
    pub default_zencast_folder: u32,
    pub default_album_folder: u32,
    pub default_text_folder: u32,
    pub cd: *mut libc::c_void,
    pub extensions: *mut LIBMTP_device_extension_t,
    pub cached: libc::c_int,
    pub next: *mut LIBMTP_mtpdevice_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_mtpdevice_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_mtpdevice_struct>(), 72usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_mtpdevice_struct>(), 4usize);
}
impl Clone for LIBMTP_mtpdevice_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_mtpdevice_t = LIBMTP_mtpdevice_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_file_struct {
    pub item_id: u32,
    pub parent_id: u32,
    pub storage_id: u32,
    pub filename: *mut libc::c_char,
    pub filesize: u64,
    pub modificationdate: libc::time_t,
    pub filetype: LIBMTP_filetype_t,
    pub next: *mut LIBMTP_file_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_file_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_file_struct>(), 36usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_file_struct>(), 4usize);
}
impl Clone for LIBMTP_file_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_file_t = LIBMTP_file_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_track_struct {
    pub item_id: u32,
    pub parent_id: u32,
    pub storage_id: u32,
    pub title: *mut libc::c_char,
    pub artist: *mut libc::c_char,
    pub composer: *mut libc::c_char,
    pub genre: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub date: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub tracknumber: u16,
    pub duration: u32,
    pub samplerate: u32,
    pub nochannels: u16,
    pub wavecodec: u32,
    pub bitrate: u32,
    pub bitratetype: u16,
    pub rating: u16,
    pub usecount: u32,
    pub filesize: u64,
    pub modificationdate: libc::time_t,
    pub filetype: LIBMTP_filetype_t,
    pub next: *mut LIBMTP_track_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_track_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_track_struct>(), 92usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_track_struct>(), 4usize);
}
impl Clone for LIBMTP_track_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_track_t = LIBMTP_track_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_playlist_struct {
    pub playlist_id: u32,
    pub parent_id: u32,
    pub storage_id: u32,
    pub name: *mut libc::c_char,
    pub tracks: *mut u32,
    pub no_tracks: u32,
    pub next: *mut LIBMTP_playlist_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_playlist_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_playlist_struct>(), 28usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_playlist_struct>(), 4usize);
}
impl Clone for LIBMTP_playlist_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_playlist_t = LIBMTP_playlist_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_album_struct {
    pub album_id: u32,
    pub parent_id: u32,
    pub storage_id: u32,
    pub name: *mut libc::c_char,
    pub artist: *mut libc::c_char,
    pub composer: *mut libc::c_char,
    pub genre: *mut libc::c_char,
    pub tracks: *mut u32,
    pub no_tracks: u32,
    pub next: *mut LIBMTP_album_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_album_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_album_struct>(), 40usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_album_struct>(), 4usize);
}
impl Clone for LIBMTP_album_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_album_t = LIBMTP_album_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_folder_struct {
    pub folder_id: u32,
    pub parent_id: u32,
    pub storage_id: u32,
    pub name: *mut libc::c_char,
    pub sibling: *mut LIBMTP_folder_t,
    pub child: *mut LIBMTP_folder_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_folder_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_folder_struct>(), 24usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_folder_struct>(), 4usize);
}
impl Clone for LIBMTP_folder_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_folder_t = LIBMTP_folder_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LIBMTP_object_struct([u8; 0]);
pub type LIBMTP_object_t = LIBMTP_object_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_filesampledata_struct {
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub filetype: LIBMTP_filetype_t,
    pub size: u64,
    pub data: *mut libc::c_char,
}
#[test]
fn bindgen_test_layout_LIBMTP_filesampledata_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_filesampledata_struct>(),
               28usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_filesampledata_struct>(),
               4usize);
}
impl Clone for LIBMTP_filesampledata_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_filesampledata_t = LIBMTP_filesampledata_struct;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct LIBMTP_devicestorage_struct {
    pub id: u32,
    pub StorageType: u16,
    pub FilesystemType: u16,
    pub AccessCapability: u16,
    pub MaxCapacity: u64,
    pub FreeSpaceInBytes: u64,
    pub FreeSpaceInObjects: u64,
    pub StorageDescription: *mut libc::c_char,
    pub VolumeIdentifier: *mut libc::c_char,
    pub next: *mut LIBMTP_devicestorage_t,
    pub prev: *mut LIBMTP_devicestorage_t,
}
#[test]
fn bindgen_test_layout_LIBMTP_devicestorage_struct() {
    assert_eq!(::std::mem::size_of::<LIBMTP_devicestorage_struct>(),
               52usize);
    assert_eq!(::std::mem::align_of::<LIBMTP_devicestorage_struct>(),
               4usize);
}
impl Clone for LIBMTP_devicestorage_struct {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LIBMTP_devicestorage_t = LIBMTP_devicestorage_struct;
pub type LIBMTP_progressfunc_t =
    ::std::option::Option<unsafe extern "C" fn(sent: u64, total: u64, data: *const libc::c_void)
                                                 -> libc::c_int>;
pub type MTPDataGetFunc = ::std::option::Option<unsafe extern "C" fn(params: *mut libc::c_void,
                                                                       priv_: *mut libc::c_void,
                                                                       wantlen: u32,
                                                                       data: *mut libc::c_uchar,
                                                                       gotlen: *mut u32)
                                                                       -> libc::c_ushort>;
pub type MTPDataPutFunc = ::std::option::Option<unsafe extern "C" fn(params: *mut libc::c_void,
                                                                       priv_: *mut libc::c_void,
                                                                       sendlen: u32,
                                                                       data: *mut libc::c_uchar,
                                                                       putlen: *mut u32)
                                                                       -> libc::c_ushort>;
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LIBMTP_event_enum {
    LIBMTP_EVENT_NONE = 0,
    LIBMTP_EVENT_STORE_ADDED = 1,
    LIBMTP_EVENT_STORE_REMOVED = 2,
    LIBMTP_EVENT_OBJECT_ADDED = 3,
    LIBMTP_EVENT_OBJECT_REMOVED = 4,
}
pub use self::LIBMTP_event_enum as LIBMTP_event_t;

extern "C" {
    #[link_name = "LIBMTP_debug"]
    pub static mut LIBMTP_debug: libc::c_int;
    pub fn LIBMTP_Set_Debug(arg1: libc::c_int);
    pub fn LIBMTP_Init();
    pub fn LIBMTP_Get_Supported_Devices_List(arg1: *const *mut LIBMTP_device_entry_t,
                                             arg2: *const libc::c_int)
                                             -> libc::c_int;
    pub fn LIBMTP_Detect_Raw_Devices(arg1: *mut *mut LIBMTP_raw_device_t,
                                     arg2: *mut libc::c_int)
                                     -> LIBMTP_error_number_t;
    pub fn LIBMTP_Check_Specific_Device(busno: libc::c_int, devno: libc::c_int) -> libc::c_int;
    pub fn LIBMTP_Open_Raw_Device(arg1: *mut LIBMTP_raw_device_t) -> *mut LIBMTP_mtpdevice_t;
    pub fn LIBMTP_Open_Raw_Device_Uncached(arg1: *mut LIBMTP_raw_device_t) -> *mut LIBMTP_mtpdevice_t;
    pub fn LIBMTP_Get_First_Device() -> *mut LIBMTP_mtpdevice_t;
    pub fn LIBMTP_Get_Connected_Devices(arg1: *mut *mut LIBMTP_mtpdevice_t) -> LIBMTP_error_number_t;
    pub fn LIBMTP_Number_Devices_In_List(arg1: *mut LIBMTP_mtpdevice_t) -> u32;
    pub fn LIBMTP_Release_Device_List(arg1: *mut LIBMTP_mtpdevice_t);
    pub fn LIBMTP_Release_Device(arg1: *mut LIBMTP_mtpdevice_t);
    pub fn LIBMTP_Dump_Device_Info(arg1: *mut LIBMTP_mtpdevice_t);
    pub fn LIBMTP_Reset_Device(arg1: *mut LIBMTP_mtpdevice_t) -> libc::c_int;
    pub fn LIBMTP_Get_Manufacturername(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Get_Modelname(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Get_Serialnumber(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Get_Deviceversion(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Get_Friendlyname(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Set_Friendlyname(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: *const libc::c_char)
                                   -> libc::c_int;
    pub fn LIBMTP_Get_Syncpartner(arg1: *mut LIBMTP_mtpdevice_t) -> *mut libc::c_char;
    pub fn LIBMTP_Set_Syncpartner(arg1: *mut LIBMTP_mtpdevice_t,
                                  arg2: *const libc::c_char)
                                  -> libc::c_int;
    pub fn LIBMTP_Get_Batterylevel(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: *const u8,
                                   arg3: *const u8)
                                   -> libc::c_int;
    pub fn LIBMTP_Get_Secure_Time(arg1: *mut LIBMTP_mtpdevice_t,
                                  arg2: *const *mut libc::c_char)
                                  -> libc::c_int;
    pub fn LIBMTP_Get_Device_Certificate(arg1: *mut LIBMTP_mtpdevice_t,
                                         arg2: *const *mut libc::c_char)
                                         -> libc::c_int;
    pub fn LIBMTP_Get_Supported_Filetypes(arg1: *mut LIBMTP_mtpdevice_t,
                                          arg2: *const *mut u16,
                                          arg3: *const u16)
                                          -> libc::c_int;
    pub fn LIBMTP_Check_Capability(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: LIBMTP_devicecap_t)
                                   -> libc::c_int;
    pub fn LIBMTP_Get_Errorstack(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_error_t;
    pub fn LIBMTP_Clear_Errorstack(arg1: *mut LIBMTP_mtpdevice_t);
    pub fn LIBMTP_Dump_Errorstack(arg1: *mut LIBMTP_mtpdevice_t);
    pub fn LIBMTP_Get_Storage(arg1: *mut LIBMTP_mtpdevice_t, arg2: libc::c_int) -> libc::c_int;
    pub fn LIBMTP_Format_Storage(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: *mut LIBMTP_devicestorage_t)
                                 -> libc::c_int;
    pub fn LIBMTP_Get_String_From_Object(arg1: *mut LIBMTP_mtpdevice_t,
                                         arg2: u32,
                                         arg3: LIBMTP_property_t)
                                         -> *mut libc::c_char;
    pub fn LIBMTP_Get_u64_From_Object(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: u32,
                                      arg3: LIBMTP_property_t,
                                      arg4: u64)
                                      -> u64;
    pub fn LIBMTP_Get_u32_From_Object(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: u32,
                                      arg3: LIBMTP_property_t,
                                      arg4: u32)
                                      -> u32;
    pub fn LIBMTP_Get_u16_From_Object(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: u32,
                                      arg3: LIBMTP_property_t,
                                      arg4: u16)
                                      -> u16;
    pub fn LIBMTP_Get_u8_From_Object(arg1: *mut LIBMTP_mtpdevice_t,
                                     arg2: u32,
                                     arg3: LIBMTP_property_t,
                                     arg4: u8)
                                     -> u8;
    pub fn LIBMTP_Set_Object_String(arg1: *mut LIBMTP_mtpdevice_t,
                                    arg2: u32,
                                    arg3: LIBMTP_property_t,
                                    arg4: *const libc::c_char)
                                    -> libc::c_int;
    pub fn LIBMTP_Set_Object_u32(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: u32,
                                 arg3: LIBMTP_property_t,
                                 arg4: u32)
                                 -> libc::c_int;
    pub fn LIBMTP_Set_Object_u16(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: u32,
                                 arg3: LIBMTP_property_t,
                                 arg4: u16)
                                 -> libc::c_int;
    pub fn LIBMTP_Set_Object_u8(arg1: *mut LIBMTP_mtpdevice_t,
                                arg2: u32,
                                arg3: LIBMTP_property_t,
                                arg4: u8)
                                -> libc::c_int;
    pub fn LIBMTP_Get_Property_Description(inproperty: LIBMTP_property_t) -> *const libc::c_char;
    pub fn LIBMTP_Is_Property_Supported(arg1: *mut LIBMTP_mtpdevice_t,
                                        arg2: LIBMTP_property_t,
                                        arg3: LIBMTP_filetype_t)
                                        -> libc::c_int;
    pub fn LIBMTP_Get_Allowed_Property_Values(arg1: *mut LIBMTP_mtpdevice_t,
                                              arg2: LIBMTP_property_t,
                                              arg3: LIBMTP_filetype_t,
                                              arg4: *mut LIBMTP_allowed_values_t)
                                              -> libc::c_int;
    pub fn LIBMTP_destroy_allowed_values_t(arg1: *mut LIBMTP_allowed_values_t);
    pub fn LIBMTP_new_file_t() -> *mut LIBMTP_file_t;
    pub fn LIBMTP_destroy_file_t(arg1: *mut LIBMTP_file_t);
    pub fn LIBMTP_Get_Filetype_Description(arg1: LIBMTP_filetype_t) -> *const libc::c_char;
    pub fn LIBMTP_Get_Filelisting(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_file_t;
    pub fn LIBMTP_Get_Filelisting_With_Callback(arg1: *mut LIBMTP_mtpdevice_t,
                                                arg2: LIBMTP_progressfunc_t,
                                                arg3: *const libc::c_void)
                                                -> *mut LIBMTP_file_t;
    pub fn LIBMTP_Get_Files_And_Folders(arg1: *mut LIBMTP_mtpdevice_t,
                                        arg2: u32,
                                        arg3: u32)
                                        -> *mut LIBMTP_file_t;
    pub fn LIBMTP_Get_Filemetadata(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> *mut LIBMTP_file_t;
    pub fn LIBMTP_Get_File_To_File(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: u32,
                                   arg3: *const libc::c_char,
                                   arg4: LIBMTP_progressfunc_t,
                                   arg5: *const libc::c_void)
                                   -> libc::c_int;
    pub fn LIBMTP_Get_File_To_File_Descriptor(arg1: *mut LIBMTP_mtpdevice_t,
                                              arg2: u32,
                                              arg3: libc::c_int,
                                              arg4: LIBMTP_progressfunc_t,
                                              arg5: *const libc::c_void)
                                              -> libc::c_int;
    pub fn LIBMTP_Get_File_To_Handler(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: u32,
                                      arg3: MTPDataPutFunc,
                                      arg4: *mut libc::c_void,
                                      arg5: LIBMTP_progressfunc_t,
                                      arg6: *const libc::c_void)
                                      -> libc::c_int;
    pub fn LIBMTP_Send_File_From_File(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: *const libc::c_char,
                                      arg3: *const LIBMTP_file_t,
                                      arg4: LIBMTP_progressfunc_t,
                                      arg5: *const libc::c_void)
                                      -> libc::c_int;
    pub fn LIBMTP_Send_File_From_File_Descriptor(arg1: *mut LIBMTP_mtpdevice_t,
                                                 arg2: libc::c_int,
                                                 arg3: *const LIBMTP_file_t,
                                                 arg4: LIBMTP_progressfunc_t,
                                                 arg5: *const libc::c_void)
                                                 -> libc::c_int;
    pub fn LIBMTP_Send_File_From_Handler(arg1: *mut LIBMTP_mtpdevice_t,
                                         arg2: MTPDataGetFunc,
                                         arg3: *mut libc::c_void,
                                         arg4: *const LIBMTP_file_t,
                                         arg5: LIBMTP_progressfunc_t,
                                         arg6: *const libc::c_void)
                                         -> libc::c_int;
    pub fn LIBMTP_Set_File_Name(arg1: *mut LIBMTP_mtpdevice_t,
                                arg2: *mut LIBMTP_file_t,
                                arg3: *const libc::c_char)
                                -> libc::c_int;
    pub fn LIBMTP_new_filesampledata_t() -> *mut LIBMTP_filesampledata_t;
    pub fn LIBMTP_destroy_filesampledata_t(arg1: *mut LIBMTP_filesampledata_t);
    pub fn LIBMTP_Get_Representative_Sample_Format(arg1: *mut LIBMTP_mtpdevice_t,
                                                   arg2: LIBMTP_filetype_t,
                                                   arg3: *mut *mut LIBMTP_filesampledata_t)
                                                   -> libc::c_int;
    pub fn LIBMTP_Send_Representative_Sample(arg1: *mut LIBMTP_mtpdevice_t,
                                             arg2: u32,
                                             arg3: *mut LIBMTP_filesampledata_t)
                                             -> libc::c_int;
    pub fn LIBMTP_Get_Representative_Sample(arg1: *mut LIBMTP_mtpdevice_t,
                                            arg2: u32,
                                            arg3: *mut LIBMTP_filesampledata_t)
                                            -> libc::c_int;
    pub fn LIBMTP_Get_Thumbnail(arg1: *mut LIBMTP_mtpdevice_t,
                                arg2: u32,
                                data: *mut *mut libc::c_uchar,
                                size: *mut libc::c_uint)
                                -> libc::c_int;
    pub fn LIBMTP_new_track_t() -> *mut LIBMTP_track_t;
    pub fn LIBMTP_destroy_track_t(arg1: *mut LIBMTP_track_t);
    pub fn LIBMTP_Get_Tracklisting(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_track_t;
    pub fn LIBMTP_Get_Tracklisting_With_Callback(arg1: *mut LIBMTP_mtpdevice_t,
                                                 arg2: LIBMTP_progressfunc_t,
                                                 arg3: *const libc::c_void)
                                                 -> *mut LIBMTP_track_t;
    pub fn LIBMTP_Get_Tracklisting_With_Callback_For_Storage(arg1: *mut LIBMTP_mtpdevice_t,
                                                             arg2: u32,
                                                             arg3: LIBMTP_progressfunc_t,
                                                             arg4: *const libc::c_void)
                                                             -> *mut LIBMTP_track_t;
    pub fn LIBMTP_Get_Trackmetadata(arg1: *mut LIBMTP_mtpdevice_t,
                                    arg2: u32)
                                    -> *mut LIBMTP_track_t;
    pub fn LIBMTP_Get_Track_To_File(arg1: *mut LIBMTP_mtpdevice_t,
                                    arg2: u32,
                                    arg3: *const libc::c_char,
                                    arg4: LIBMTP_progressfunc_t,
                                    arg5: *const libc::c_void)
                                    -> libc::c_int;
    pub fn LIBMTP_Get_Track_To_File_Descriptor(arg1: *mut LIBMTP_mtpdevice_t,
                                               arg2: u32,
                                               arg3: libc::c_int,
                                               arg4: LIBMTP_progressfunc_t,
                                               arg5: *const libc::c_void)
                                               -> libc::c_int;
    pub fn LIBMTP_Get_Track_To_Handler(arg1: *mut LIBMTP_mtpdevice_t,
                                       arg2: u32,
                                       arg3: MTPDataPutFunc,
                                       arg4: *mut libc::c_void,
                                       arg5: LIBMTP_progressfunc_t,
                                       arg6: *const libc::c_void)
                                       -> libc::c_int;
    pub fn LIBMTP_Send_Track_From_File(arg1: *mut LIBMTP_mtpdevice_t,
                                       arg2: *const libc::c_char,
                                       arg3: *const LIBMTP_track_t,
                                       arg4: LIBMTP_progressfunc_t,
                                       arg5: *const libc::c_void)
                                       -> libc::c_int;
    pub fn LIBMTP_Send_Track_From_File_Descriptor(arg1: *mut LIBMTP_mtpdevice_t,
                                                  arg2: libc::c_int,
                                                  arg3: *const LIBMTP_track_t,
                                                  arg4: LIBMTP_progressfunc_t,
                                                  arg5: *const libc::c_void)
                                                  -> libc::c_int;
    pub fn LIBMTP_Send_Track_From_Handler(arg1: *mut LIBMTP_mtpdevice_t,
                                          arg2: MTPDataGetFunc,
                                          arg3: *mut libc::c_void,
                                          arg4: *const LIBMTP_track_t,
                                          arg5: LIBMTP_progressfunc_t,
                                          arg6: *const libc::c_void)
                                          -> libc::c_int;
    pub fn LIBMTP_Update_Track_Metadata(arg1: *mut LIBMTP_mtpdevice_t,
                                        arg2: *const LIBMTP_track_t)
                                        -> libc::c_int;
    pub fn LIBMTP_Track_Exists(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> libc::c_int;
    pub fn LIBMTP_Set_Track_Name(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: *mut LIBMTP_track_t,
                                 arg3: *const libc::c_char)
                                 -> libc::c_int;
    pub fn LIBMTP_new_folder_t() -> *mut LIBMTP_folder_t;
    pub fn LIBMTP_destroy_folder_t(arg1: *mut LIBMTP_folder_t);
    pub fn LIBMTP_Get_Folder_List(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_folder_t;
    pub fn LIBMTP_Get_Folder_List_For_Storage(arg1: *mut LIBMTP_mtpdevice_t,
                                              arg2: u32)
                                              -> *mut LIBMTP_folder_t;
    pub fn LIBMTP_Find_Folder(arg1: *mut LIBMTP_folder_t, arg2: u32) -> *mut LIBMTP_folder_t;
    pub fn LIBMTP_Create_Folder(arg1: *mut LIBMTP_mtpdevice_t,
                                arg2: *mut libc::c_char,
                                arg3: u32,
                                arg4: u32)
                                -> u32;
    pub fn LIBMTP_Set_Folder_Name(arg1: *mut LIBMTP_mtpdevice_t,
                                  arg2: *mut LIBMTP_folder_t,
                                  arg3: *const libc::c_char)
                                  -> libc::c_int;
    pub fn LIBMTP_new_playlist_t() -> *mut LIBMTP_playlist_t;
    pub fn LIBMTP_destroy_playlist_t(arg1: *mut LIBMTP_playlist_t);
    pub fn LIBMTP_Get_Playlist_List(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_playlist_t;
    pub fn LIBMTP_Get_Playlist(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> *mut LIBMTP_playlist_t;
    pub fn LIBMTP_Create_New_Playlist(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: *const LIBMTP_playlist_t)
                                      -> libc::c_int;
    pub fn LIBMTP_Update_Playlist(arg1: *mut LIBMTP_mtpdevice_t,
                                  arg2: *const LIBMTP_playlist_t)
                                  -> libc::c_int;
    pub fn LIBMTP_Set_Playlist_Name(arg1: *mut LIBMTP_mtpdevice_t,
                                    arg2: *mut LIBMTP_playlist_t,
                                    arg3: *const libc::c_char)
                                    -> libc::c_int;
    pub fn LIBMTP_new_album_t() -> *mut LIBMTP_album_t;
    pub fn LIBMTP_destroy_album_t(arg1: *mut LIBMTP_album_t);
    pub fn LIBMTP_Get_Album_List(arg1: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_album_t;
    pub fn LIBMTP_Get_Album_List_For_Storage(arg1: *mut LIBMTP_mtpdevice_t,
                                             arg2: u32)
                                             -> *mut LIBMTP_album_t;
    pub fn LIBMTP_Get_Album(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> *mut LIBMTP_album_t;
    pub fn LIBMTP_Create_New_Album(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: *const LIBMTP_album_t)
                                   -> libc::c_int;
    pub fn LIBMTP_Update_Album(arg1: *mut LIBMTP_mtpdevice_t,
                               arg2: *const LIBMTP_album_t)
                               -> libc::c_int;
    pub fn LIBMTP_Set_Album_Name(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: *mut LIBMTP_album_t,
                                 arg3: *const libc::c_char)
                                 -> libc::c_int;
    pub fn LIBMTP_Delete_Object(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> libc::c_int;
    pub fn LIBMTP_Set_Object_Filename(arg1: *mut LIBMTP_mtpdevice_t,
                                      arg2: u32,
                                      arg3: *mut libc::c_char)
                                      -> libc::c_int;
    pub fn LIBMTP_GetPartialObject(arg1: *mut LIBMTP_mtpdevice_t,
                                   arg2: u32,
                                   arg3: u64,
                                   arg4: u32,
                                   arg5: *mut *mut libc::c_uchar,
                                   arg6: *mut libc::c_uint)
                                   -> libc::c_int;
    pub fn LIBMTP_SendPartialObject(arg1: *mut LIBMTP_mtpdevice_t,
                                    arg2: u32,
                                    arg3: u64,
                                    arg4: *mut libc::c_uchar,
                                    arg5: libc::c_uint)
                                    -> libc::c_int;
    pub fn LIBMTP_BeginEditObject(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> libc::c_int;
    pub fn LIBMTP_EndEditObject(arg1: *mut LIBMTP_mtpdevice_t, arg2: u32) -> libc::c_int;
    pub fn LIBMTP_TruncateObject(arg1: *mut LIBMTP_mtpdevice_t,
                                 arg2: u32,
                                 arg3: u64)
                                 -> libc::c_int;
    pub fn LIBMTP_Read_Event(arg1: *mut LIBMTP_mtpdevice_t,
                             arg2: *mut LIBMTP_event_t,
                             arg3: *mut u32)
                             -> libc::c_int;
}
