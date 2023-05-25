pub fn convert_fah_to_celcius(fah_tmp: i32) -> i32 {
  let celcius = fah_tmp - 32;
  let celcius = celcius * 5 / 9;

  celcius
}