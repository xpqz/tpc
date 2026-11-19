program DosPackUnpackTime;
uses Dos;
var
  dt: DateTime;
  t: LongInt;

procedure Show(d: DateTime);
begin
  WriteLn(d.Year, ' ', d.Month, ' ', d.Day, ' ', d.Hour, ' ', d.Min, ' ', d.Sec);
end;

procedure Pack(y, mo, d, h, mi, s: Word);
begin
  dt.Year := y; dt.Month := mo; dt.Day := d;
  dt.Hour := h; dt.Min := mi; dt.Sec := s;
  PackTime(dt, t);
  WriteLn(t);
end;

begin
  Pack(1990, 12, 31, 23, 59, 58);
  Pack(1980, 1, 1, 0, 0, 0);
  Pack(2099, 12, 31, 23, 59, 59);
  Pack(2000, 2, 29, 12, 30, 15);
  UnpackTime(t, dt); Show(dt);
  UnpackTime($12345678, dt); Show(dt);
  UnpackTime(0, dt); Show(dt);
  UnpackTime(-1, dt); Show(dt);
  { out-of-range fields are added in without masking; the year wraps in 7 bits }
  Pack(1970, 13, 40, 30, 70, 61);
  Pack(1980, 13, 40, 30, 70, 61);
  Pack(1980, 1, 1, 0, 0, 1);
  Pack(2108, 1, 1, 0, 0, 0);
  Pack(1979, 1, 1, 0, 0, 0);
  Pack(2107, 1, 1, 0, 0, 0);
end.
