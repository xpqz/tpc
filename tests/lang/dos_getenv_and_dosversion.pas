program EnvironmentNames;
uses Dos;
var s: string; i: Integer; v: Word;
begin
  s := GetEnv('NO_SUCH_VARIABLE');
  Writeln(Length(s));
  Writeln(Length(GetEnv('')));
  Writeln(Length(GetEnv('no_such_either')));
  Writeln(Length(EnvStr(0)));
  Writeln(Length(EnvStr(EnvCount + 1)));
  Writeln(Length(EnvStr(-1)));
  Writeln(Length(EnvStr(30000)));
  Writeln(EnvCount >= 0);
  v := DosVersion;
  Writeln(Lo(v), ' ', Hi(v));
  Writeln(DosVersion = v);
end.
