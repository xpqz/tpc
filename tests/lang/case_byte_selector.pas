program CaseByteSelector;
var b: Byte;
procedure Show(b: Byte);
begin
  case b of
    0: Write('min');
    1..127: Write('low');
    128..254: Write('high');
    255: Write('max');
  end;
  WriteLn(' ', b)
end;
begin
  Show(0);
  Show(1);
  Show(127);
  Show(128);
  Show(254);
  Show(255)
end.
