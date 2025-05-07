/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

/// The response data from website.
#[derive(Debug, serde::Deserialize)]
struct R {
    id: String,
    nama: String,
}

/// Placeholder for the data.
#[derive(Clone)]
pub struct NamedField {
    pub name: String,
}

/// Province data.
#[derive(Clone)]
pub struct ProvinceData {
    pub name: String,
    pub cities: Vec<NamedField>,
}

/// Get all province data.
pub async fn get_province_data() -> Result<Vec<ProvinceData>, reqwest::Error> {
    let base = "https://ibnux.github.io/data-indonesia";
    let prov = format!("{}/provinsi.json", base);
    let res = reqwest::get(&prov).await?.json::<Vec<R>>().await?;
    let mut data: Vec<ProvinceData> = Vec::new();
    for r in res {
        let cities = reqwest::get(format!("{}/kabupaten/{}.json", base, r.id))
            .await?
            .json::<Vec<R>>()
            .await?;
        data.push(ProvinceData {
            name: r.nama,
            cities: cities
                .iter()
                .map(|r| NamedField {
                    name: r.nama.clone(),
                })
                .collect(),
        });
    }
    Ok(data)
}
