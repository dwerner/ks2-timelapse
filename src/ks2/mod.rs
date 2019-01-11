// {"errCode": 412,"errMsg": "Precondition Failed"}
#[derive(Serialize, Deserialize, Debug)]
pub struct KS2Response {
    #[serde(rename="errCode")]
    pub err_code: u32,
    #[serde(rename="errMsg")]
    pub err_msg: String,
}

// {"errCode": 200,
// "errMsg": "OK",
// "focused": true,
// "focusCenters": [],
// "captured": false}
#[derive(Serialize, Deserialize, Debug)]
pub struct ShootResponse {

    #[serde(rename="errCode")]
    pub err_code: u32,
    #[serde(rename="errMsg")]
    pub err_msg: String,

    pub focused: bool,

    //pub focus_centers: Vec<?>,

    pub captured: bool
}

//
// {"errCode": 200,
// "errMsg": "OK",
// "dirs": [
//  {"name":"100_0101", files: [ "IMGXYZ.JPG" ] }
// ]}
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotosResponse {
    pub err_code: u32,
    pub err_msg: String,
    pub dirs: Vec<ImgDir>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImgDir {
    pub name: String,
    pub files: Vec<String>
}
