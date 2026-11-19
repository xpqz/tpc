program UtilsTrim;
uses Utils;
var
  Long, Padded: String;
  I: Integer;

procedure Show(S: String);
begin
  WriteLn('[', TrimLeft(S), '] [', TrimRight(S), '] [', Trim(S), '] ', Length(Trim(S)));
end;

begin
  Show('');
  Show(' ');
  Show('     ');
  Show('x');
  Show('  x');
  Show('x  ');
  Show('  x y  ');
  Show(#9#9'tab'#9);
  Show(#1#2'ctl'#31#32);
  Show(#0'nul'#0);
  Show(#255'high'#255);
  Show('a'#13#10);
  Show(' '#9' '#10);
  Long := '';
  for I := 1 to 255 do
    Long := Long + ' ';
  Show(Long);
  Padded := Long;
  Padded[128] := 'Z';
  Show(Padded);
  Padded := Long;
  Padded[1] := 'L';
  Padded[255] := 'R';
  Show(Padded);
  WriteLn(Length(TrimLeft(Padded)), ' ', Length(TrimRight(Padded)), ' ', Length(Trim(Padded)));
  Padded := '';
  for I := 1 to 255 do
    Padded := Padded + 'q';
  WriteLn(Length(Trim(Padded)), ' ', Trim(Padded) = Padded);
end.
