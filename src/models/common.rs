use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    pub address: String,
    pub name: Option<String>,
}

/*ZeptoMail does not support the following extensions as attachments:
ade, adp, app, asp, bas, bat, cer, chm, cmd, com, cpl, crt, csh, der, exe, fxp, gadget, hlp, hpj, hta, inf, ins, 
isp, js, jse, ksh, lib, lnk, mad, maf, mag, mam, maq, mar, mas, mat, mau, mav, maw, mda, mdb, mdt, mdw, mdz, msc, 
msh, msh1, msh1xml, msh2, msh2xml, msi, msp, mst, ops, osd, pcd, plg, prf, prg, ps1, ps1xml, ps2, ps2xml, psc1, 
psc2, pst, reg, scf, scr, sct, shb, shs, sys, tmp, url, vb, vbe, vbp, vbs, vsmacros, vsw, vxd, ws, wsc, wsf, wsh, xnk */
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub content: Option<String>,       // Base64 encoded content
    pub mime_type: Option<String>,
    pub file_cache_key: Option<String>, // File cache key for an uploaded file
}

//The additional headers to be sent in the email for your reference purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct MimeHeaders {
    pub headers: std::collections::HashMap<String, String>,
}
