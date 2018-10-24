// {"errCode": 412,"errMsg": "Precondition Failed"}

#[derive(Serialize, Deserialize, Debug)]
pub struct KS2Response {
  errCode: u64
}

// {"errCode": 200,
// "errMsg": "OK",
// "focused": true,
// "focusCenters": [],
// "captured": false}
#[derive(Serialize, Deserialize, Debug)]
pub struct ShootResponse {
    err_code: u32,
    err_msg: String,
    focused: bool,
    //focus_centers: Vec<?>,
    captured: bool
}

//
// {"errCode": 200,
// "errMsg": "OK",
// "dirs": [
//  {"name":"100_0101", files: [ "IMGXYZ.JPG" ] }
// ]}
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotosResponse {
    err_code: u32,
    err_msg: String,
    dirs: Vec<ImgDir>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImgDir {
    name: String,
    files: Vec<String>
}
