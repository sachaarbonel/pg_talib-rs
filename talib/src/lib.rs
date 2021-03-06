fn ema(inReal: &[f64], inTimePeriod: i64, k1: f64) -> Vec<f64> {
    let mut outReal = vec![0.0; inReal.len()];
    let lookbackTotal = inTimePeriod - 1;
    let startIdx = lookbackTotal;
    let mut today = startIdx - lookbackTotal;
    let mut i = inTimePeriod;
    let mut tempReal = 0.0;
    while i > 0 {
        let inRealToday = inReal[today as usize];
        tempReal += inRealToday;
        today = today + 1;
        i = i - 1;
    }
    let mut prevMA = tempReal / inTimePeriod as f64;
    while today <= startIdx {
        prevMA = (((inReal[today as usize]) - prevMA) * k1) + prevMA;
        today = today + 1;
    }
    outReal[startIdx as usize] = prevMA;
    let mut outIdx = startIdx + 1;
    while today < inReal.len() as i64 {
        prevMA = (((inReal[today as usize]) - prevMA) * k1) + prevMA;
        outReal[outIdx as usize - 1] = prevMA;
        today = today + 1;
        outIdx = today + 1;
    }
    outReal
}

pub fn Macd(
    inReal: &[f64],
    mut inFastPeriod: i64,
    mut inSlowPeriod: i64,
    inSignalPeriod: i64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    if inSlowPeriod < inFastPeriod {
        inSlowPeriod = inFastPeriod;
        inFastPeriod = inSlowPeriod;
    }
    let mut k1 = 0.0;
    let mut k2 = 0.0;
    if (inSlowPeriod != 0) {
        k1 = 2.0 / (inSlowPeriod + 1) as f64;
    } else {
        inSlowPeriod = 26;
        k1 = 0.075;
    }
    if (inFastPeriod != 0) {
        k2 = 2.0 / (inFastPeriod + 1) as f64;
    } else {
        inFastPeriod = 12;
        k2 = 0.15;
    }
    let lookbackSignal = inSignalPeriod - 1;
    let mut lookbackTotal = lookbackSignal;
    lookbackTotal += inSlowPeriod - 1;
    let mut fastEMABuffer = ema(inReal, inFastPeriod, k2);
    let slowEMABuffer = ema(inReal, inSlowPeriod, k1);

    for i in 0..fastEMABuffer.len() {
        let fastEMABufferElt = fastEMABuffer[i as usize];
        let slowEMABufferElt = slowEMABuffer[i as usize];
        fastEMABuffer[i] = fastEMABufferElt - slowEMABufferElt;
    }
    let mut outMACD = vec![0.0; inReal.len()];
    for i in lookbackTotal - 1..fastEMABuffer.len() as i64 {
        outMACD[i as usize] = fastEMABuffer[i as usize];
    }
    let outMACDSignal = ema(
        &outMACD,
        inSignalPeriod,
        (2.0 / (inSignalPeriod + 1) as f64),
    );
    let mut outMACDHist = vec![0.0; inReal.len()];
    for i in lookbackTotal..outMACDHist.len() as i64 {
        let outMacdElt = outMACD[i as usize];
        let outMACDSignalElt = outMACDSignal[i as usize];
        outMACDHist[i as usize] = outMacdElt - outMACDSignalElt;
    }
    (outMACD, outMACDSignal, outMACDHist)
}

#[cfg(test)]
mod tests {
    use crate::Macd;

    #[test]
    fn it_works() {
        let Close = vec![
        201.28, 197.64, 195.78, 198.22, 201.74, 200.12, 198.55, 197.99, 196.8, 195.0, 197.55,
        197.97, 198.97, 201.93, 200.83, 201.3, 198.64, 196.09, 197.91, 195.42, 197.84, 200.7,
        199.93, 201.95, 201.39, 200.49, 202.63, 202.75, 204.7, 205.54, 205.86, 205.88, 205.73,
        206.97, 206.94, 207.53, 207.35, 207.11, 206.4, 207.7, 206.85, 205.98, 206.2, 203.3, 204.15,
        200.84, 200.37, 202.91, 201.67, 204.36, 203.76, 206.2, 205.26, 207.08, 206.67, 205.51,
        202.5, 202.02, 202.48, 204.95, 203.16, 202.44, 203.17, 204.54, 204.0, 204.68, 205.59,
        206.71, 205.78, 206.17, 207.1, 207.04, 204.66, 206.52, 206.28, 207.29, 207.81, 208.3,
        207.43, 208.09, 207.23, 205.16, 207.38, 207.97, 205.59, 204.74, 205.56, 208.27, 207.27,
        206.65, 206.69, 208.85, 209.07, 209.72, 209.65, 209.51, 210.12, 209.62, 207.36, 209.33,
        209.09, 207.79, 208.22, 208.01, 208.56, 206.8, 206.45, 205.18, 205.15, 207.62, 208.28,
        206.68, 205.79, 206.92, 207.25, 209.41, 208.48, 209.55, 209.71, 208.18, 207.54, 207.5,
        203.15, 203.57, 205.21, 205.03, 204.43, 205.71, 202.27, 202.63, 205.19, 207.44, 208.35,
        208.28, 209.95, 210.12, 210.24, 209.42, 209.03, 207.86, 205.7, 204.5, 207.02, 208.44,
        208.49, 208.17, 207.47, 207.06, 207.75, 206.05, 205.65, 208.24, 206.35, 206.61, 206.35,
        207.1, 208.26, 207.66, 206.02, 201.71, 195.64, 187.4, 185.2, 192.31, 197.07, 197.08,
        195.48, 189.65, 193.25, 193.39, 190.46, 195.25, 192.64, 193.68, 194.56, 193.84, 196.26,
        197.97, 197.52, 194.29, 195.3, 192.75, 192.45, 191.76, 191.73, 186.9, 187.01, 190.5,
        190.99, 193.85, 197.3, 196.62, 198.23, 200.02, 200.14, 200.33, 199.07, 198.11, 201.15,
        202.07, 202.17, 201.91, 200.66, 204.05, 206.28, 205.78, 205.38, 207.71, 207.59, 206.7,
        209.15, 209.75, 209.12, 208.91, 208.8, 206.85, 207.33, 206.51, 203.63, 201.34, 204.4,
        204.25, 207.5, 207.32, 208.07, 207.83, 208.11, 208.08, 208.32, 207.46, 209.43, 207.3,
        204.39, 208.38, 207.12, 205.73, 204.13, 204.65, 200.69, 201.7, 203.82, 206.8, 203.65,
        200.02, 201.67, 203.5, 206.02, 205.68, 205.21, 207.4, 205.93, 203.87,
    ];
    let (macd, macdsignal, macdhist) = Macd(&Close, 12, 26, 9);
    let expected_result = vec![
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        2.0095391541944707,
        2.1434930548656723,
        2.221622258469978,
        2.3045824919665847,
        2.3289578032022007,
        2.302369140458552,
        2.198661565483235,
        2.1960569233745844,
        2.101183691826492,
        1.9335059897096016,
        1.797650171929007,
        1.4393853554692555,
        1.2100965635937087,
        0.7526184894622361,
        0.34812577053344285,
        0.2298693580332838,
        0.0356813498575832,
        0.09772001752781989,
        0.09734894498993185,
        0.2905927035999696,
        0.36369715603120767,
        0.5620132124256827,
        0.6782777784051177,
        0.6691029764983227,
        0.41417567612694484,
        0.17143576072342626,
        0.015996388134084327,
        0.09106814592723822,
        0.006055213760276956,
        -0.11805526259570343,
        -0.15571383794474514,
        -0.0741561538114297,
        -0.05248952203962176,
        0.019328944699452677,
        0.14796929391440017,
        0.33641436188719354,
        0.4060346930694436,
        0.4870644717099992,
        0.6191867829331272,
        0.7108588298123948,
        0.5847231533809065,
        0.6276114621211377,
        0.634915846893449,
        0.7139729478188315,
        0.8092573095373723,
        0.9137763130543135,
        0.9158493065441746,
        0.9596859902579524,
        0.9144903892900231,
        0.7035309823859279,
        0.7073259657814219,
        0.7493040756693006,
        0.583796280559568,
        0.3796657142421509,
        0.28082078947218747,
        0.4163602838607119,
        0.4380352691068481,
        0.4005665507771141,
        0.36983673954125607,
        0.5138537052164054,
        0.6383814851018883,
        0.7805228341847794,
        0.877408246324535,
        0.932148574965936,
        1.0130744897138868,
        1.025046935858711,
        0.8424607106347537,
        0.8469591300674324,
        0.8216862777133542,
        0.6888179600260855,
        0.6111710677067208,
        0.5266195587076936,
        0.49824878033288655,
        0.3299440533402844,
        0.16640110138490627,
        -0.06493769118850423,
        -0.24783916467262657,
        -0.19127662492746822,
        -0.09213188153819374,
        -0.14103976769715132,
        -0.24874777232088263,
        -0.24015720303953003,
        -0.2043650794938685,
        -0.0016861783740864666,
        0.08293901399466108,
        0.23365175429503893,
        0.36183237129051804,
        0.3360840527395226,
        0.2610267199404177,
        0.19605557065406742,
        -0.2040905499623591,
        -0.4817652535796526,
        -0.5630002464014012,
        -0.6345889131830802,
        -0.7313083653546926,
        -0.6966434533483437,
        -0.9359613319377047,
        -1.084076919642314,
        -0.983551218617265,
        -0.7140960258641087,
        -0.42225396856903785,
        -0.19437481862479444,
        0.119597447783093,
        0.37778543488008154,
        0.5853368728064368,
        0.6758647871611743,
        0.7079779418588998,
        0.631736362868736,
        0.3924959649377513,
        0.10485752251389613,
        0.07933045198222999,
        0.17170291293911077,
        0.2461063747949197,
        0.2760679887868207,
        0.24055565799659462,
        0.17728467908119683,
        0.1807357961237983,
        0.04576745994776843,
        -0.0924073613861367,
        0.006998774165793975,
        -0.06596791477974762,
        -0.10164303671965058,
        -0.14917604636806914,
        -0.12488793658985742,
        -0.011899953868010016,
        0.028895758564999596,
        -0.07029729589422118,
        -0.49102920698453545,
        -1.2992832204372462,
        -2.575045957809749,
        -3.7207284904199867,
        -4.008762058813545,
        -3.8090303866567865,
        -3.608339983837027,
        -3.537618557310566,
        -3.9069665159316003,
        -3.864638375068523,
        -3.776265747610921,
        -3.8977257603138185,
        -3.566359949635853,
        -3.4743057110746633,
        -3.2796272539560505,
        -3.0195272974790157,
        -2.8387706102394077,
        -2.471753027812497,
        -2.0196252577878226,
        -1.678275862380616,
        -1.6493750441854615,
        -1.527365855637271,
        -1.6177875357703044,
        -1.6941260311934059,
        -1.789671849189375,
        -1.846527725406247,
        -2.2553290218678512,
        -2.5411382538891303,
        -2.4576994669669148,
        -2.3252309077888924,
        -1.9667984935293248,
        -1.3883480436846014,
        -0.9735695084081328,
        -0.5090727128678054,
        0.003442699169426078,
        0.4145195296923987,
        0.7470210974275631,
        0.8985021422636237,
        0.9303633353145244,
        1.1872304998379946,
        1.4483403238967014,
        1.6443854530150759,
        1.7585018880450036,
        1.7281543441163478,
        1.955111062188024,
        2.288537424640907,
        2.483802666737688,
        2.5765739798706306,
        2.805764336543234,
        2.9437822588759843,
        2.947371431542649,
        3.112036778429143,
        3.2534465313638066,
        3.2769049243312622,
        3.2411882245270647,
        3.1674935084465403,
        2.9181031353527374,
        2.727747679107125,
        2.4821102398365724,
        2.0316295083301554,
        1.4728582285649452,
        1.2623923173122762,
        1.0711453233984969,
        1.1683604099515321,
        1.216852453148931,
        1.3008065900910424,
        1.3326133174378185,
        1.3646828238498188,
        1.3718634021059586,
        1.3810007394621096,
        1.3038177332373095,
        1.385639392369029,
        1.2640393886418053,
        0.9222269710362241,
        0.9622061085699158,
        0.8820506623804079,
        0.6983157610937099,
        0.41877057334949086,
        0.23646285561997615,
        -0.22496299565662525,
        -0.5033454945696008,
        -0.5465985009022916,
        -0.33653634346038075,
        -0.4194046093567465,
        -0.7691229987700865,
        -0.9027300218287451,
        -0.8511374612982934,
        -0.5999907262809927,
        -0.4235083472642316,
        -0.31790524863555447,
        -0.056844071662823126,
        0.031073876924494925,
        -0.06472914201052049,
    ];
    assert_eq!(macd, expected_result);
    }
}
