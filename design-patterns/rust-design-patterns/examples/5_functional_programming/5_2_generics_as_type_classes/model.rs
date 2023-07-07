#![allow(dead_code)]
use std::path::PathBuf;

/// 这种设计可能工作得很好。但现在，假设你需要支持添加协议特定的元数据
/// 例如，对于NFS，你想确定他们的挂载点是什么，以便执行额外的安全规则

mod nfs {
    #[derive(Clone)]
    pub struct AuthInfo;
}

mod bootp {
    pub struct AuthInfo;
}

pub enum AuthInfo {
    Nfs(nfs::AuthInfo),
    Bootp(bootp::AuthInfo),
}

struct FileDownloadRequest {
    file_name: PathBuf,
    authentication: AuthInfo,
    mount_point: Option<PathBuf>,
}
impl FileDownloadRequest {
    // ... 其他方法 ...

    /// 如果有NFS请求，获取一个NFS挂载点。
    /// 否则返回None。
    pub fn mount_point(&self) -> Option<&PathBuf> {
        self.mount_point.as_ref()
    }
}
