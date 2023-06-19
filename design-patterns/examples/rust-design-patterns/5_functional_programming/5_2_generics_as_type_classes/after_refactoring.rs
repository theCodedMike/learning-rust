#![allow(dead_code)]
use std::path::{Path, PathBuf};

mod nfs {
    #[derive(Clone)]
    pub(crate) struct AuthInfo(String); // NFS会话管理给省了
}

mod bootp {
    pub(crate) struct AuthInfo(); // bootp没验证机制
}

// private module, lest outside users invent their own protocol kinds!
mod proto_trait {
    use super::{bootp, nfs};
    use std::path::{Path, PathBuf};

    pub(crate) trait ProtoKind {
        type AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub(crate) fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp(); // 没有附加元数据

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}

use proto_trait::ProtoKind; // 保持内部，以防止 impl
pub use proto_trait::{Bootp, Nfs}; // 重导出，这样调用者能看到它们

struct FileDownloadRequest<P: ProtoKind> {
    file_name: PathBuf,
    protocol: P,
}

// 把所有共同的API部分放进一个泛型实现块
impl<P: ProtoKind> FileDownloadRequest<P> {
    fn file_path(&self) -> &Path {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

// all protocol-specific impls go into their own block
impl FileDownloadRequest<Nfs> {
    fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}
