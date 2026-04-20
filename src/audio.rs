use symphonia::core::audio::{AudioBuffer, Signal};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::path::Path;

fn audio() -> Result<(), Box<dyn std::error::Error>> {
    // ====================== 1. 打开音频文件 ======================
    let path = Path::new("assets/qwen_tts_output.wav"); // 替换成你的 WAV 文件路径
    let src = std::fs::File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // ====================== 2. 探测格式 + 初始化读取器 ======================
    let mut hint = Hint::new();
    hint.with_extension("wav"); // 告诉 Symphonia 这是 WAV 文件

    let format_opts = FormatOptions::default();
    let meta_opts = MetadataOptions::default();

    // 探测文件格式并创建格式读取器
    let prob = symphonia::default::get_probe();
    let probed = prob.format(&hint, mss, &format_opts, &meta_opts)?;

    let mut reader = probed.format;
    let track = reader.default_track().ok_or("无音频轨道")?; // 获取默认音频轨道

    // ====================== 3. 初始化解码器 ======================
    let codec = symphonia::default::get_codecs();
    let decode_opts = DecoderOptions::default();
    let mut decoder = codec.make(&track.codec_params, &decode_opts)?;

    // ====================== 4. 打印音频信息 ======================
    let params = &track.codec_params;
    println!("=== 音频信息 ===");
    println!("采样率: {} Hz", params.sample_rate.unwrap());
    println!("声道数: {}", params.channels.unwrap().count());
    println!("位深度: {:?}", params.bits_per_sample);
    println!("编码格式: {:?}", params.codec);

    // 存储所有解码后的采样（f32 格式，-1.0 ~ 1.0）
    let mut all_samples: Vec<f32> = Vec::new();

    // ====================== 5. 循环解码 ======================
    loop {
        // 读取一帧数据包
        let packet = match reader.next_packet() {
            Ok(pkt) => pkt,
            Err(Error::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break, // 文件结束
            Err(e) => return Err(Box::new(e)),
        };

        // 解码数据包 → PCM 音频缓冲区
        let decoded = decoder.decode(&packet)?;

        // 将解码数据转为 f32 格式（Symphonia 自动处理格式转换）
        let mut buf = AudioBuffer::<f32>::new(decoded.capacity() as u64, decoded.spec().clone());
        decoded.convert(&mut buf);

        // 把所有声道的采样存入数组
        let planes = buf.planes();
        let plane = planes.planes()[0]; // 0=左声道/单声道，1=右声道

        // 复制采样到总列表
        all_samples.extend_from_slice(plane);
    }

    // ====================== 6. 输出结果 ======================
    println!("\n=== 解码完成 ===");
    println!("总采样点数: {}", all_samples.len());
    println!("前 10 个采样值: {:?}", &all_samples[0..10.min(all_samples.len())]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_audio() {
        audio().unwrap();
    }
}
