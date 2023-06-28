// from Programming Rust: Fast, Safe Systems Development

use num::Complex;
use std::str::FromStr;
use std::env;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 5
    {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {}mandel.png 1000x750 -1.20, 0.35 -1.0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = ParsePair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = ParseComplex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = ParseComplex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner|
        {
            for(i, band) in bands.into_iter().enumerate()
            {
                let top = rows_per_band * 1;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = PixelToPoint(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = PixelToPoint(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| 
                {
                    Render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    WriteImage(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// c가 맹델브로 집합에 속하는지 아닌지를 판단
/// 반복은 limit로 제한함
/// 망델로브 집합에 속한다면 Some(), 속하지 않는다면 None을 return
fn EscapeTime(c: Complex<f64>, limit: usize) -> Option<usize>
{
    let mut z = Complex{re:0.0, im:0.0};
    for i in 0..limit
    {
        if z.norm_sqr() > 4.0
        {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// s를 400x600 혹은 좌표 쌍으로 파싱
/// s가 올바르다면, Some<(x, y)>,아니라면 None을 return
fn ParsePair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)>
{
    match s.find(separator)
    {
        None => None,
        Some(index) =>
        {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
            {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn TestParsePair()
{
    // 굳이 타입 적어줄 필요 없음  
    assert_eq!(ParsePair::<i32>("", ','), None);
    assert_eq!(ParsePair::<i32>("10,", ','), None);
    assert_eq!(ParsePair::<i32>(",10", ','), None);
    assert_eq!(ParsePair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(ParsePair::<i32>("10,20xy", ','), None);
    assert_eq!(ParsePair::<f64>("0.5x", 'x'), None);
    assert_eq!(ParsePair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// 쉼표로 구분된 부동소수점 수 쌍을 복소수로 파싱
fn ParseComplex(s: &str) -> Option<Complex<f64>>
{
    match ParsePair(s, ',')
    {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn TestParseComplex()
{
    assert_eq!(ParseComplex("1.25,-0.0625"),
                // 축약 표기
                Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(ParseComplex(", -0.0625"), None);
}

/// 결과 이미지의 픽셀 좌표가 주어지면 복소 평면 위의 점을 return
fn PixelToPoint(bounds: (usize, usize),
                pixel: (usize, usize),
                upper: Complex<f64>,
                lower: Complex<f64>) -> Complex<f64>
{
    let (width, height) = (lower.re - upper.re, upper.im - lower.im);
    Complex { re: upper.re + pixel.0 as f64 * width / bounds.0 as f64, 
              im: upper.im + pixel.1 as f64 * height / bounds.1 as f64}
}

#[test]
fn TestPixelToPoint()
{
    assert_eq!(PixelToPoint((100, 200), (25, 175),
                            Complex { re: -1.0, im: 1.0 },
                            Complex{re: 1.0, im: -1.0}),
                            Complex{re: -0.5, im: -0.75});
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 렌더링
fn Render(pixels: &mut [u8]
        , bounds: (usize, usize)
        , upper_left: Complex<f64>
        , lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1
    {
        for column in 0..bounds.0
        {
            let point = PixelToPoint(bounds, (column, row), 
                                        upper_left, lower_right);
            pixels[row * bounds.0 + column] = match EscapeTime(point, 255)
            {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// bonuds 크기의 pixels 버퍼를 filename 파일에 기록
fn WriteImage(filename: &str, pixels: &[u8], bounds: (usize, usize))
                -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels 
                  , bounds.0 as u32
                  , bounds.1 as u32
                  , ColorType::Gray(8))?;

    Ok(())
}