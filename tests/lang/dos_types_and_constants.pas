program DosTypesAndConstants;
uses Dos;
var
  dt: DateTime;
  sr: SearchRec;
  p: PathStr;
  d: DirStr;
  n: NameStr;
  e: ExtStr;
begin
  WriteLn(SizeOf(DateTime), ' ', SizeOf(SearchRec));
  WriteLn(SizeOf(PathStr), ' ', SizeOf(DirStr), ' ', SizeOf(NameStr), ' ', SizeOf(ExtStr));
  WriteLn(ReadOnly, ' ', Hidden, ' ', SysFile, ' ', VolumeID);
  WriteLn(Directory, ' ', Archive, ' ', AnyFile);
  WriteLn(fmClosed, ' ', fmInput, ' ', fmOutput, ' ', fmInOut);
  sr.Attr := Directory or Archive;
  sr.Size := 123456;
  sr.Name := 'FILE.TXT';
  WriteLn(sr.Attr, ' ', sr.Size, ' ', sr.Name, ' ', sr.Attr and Hidden);
  dt.Month := 7;
  p := 'C:\X.PAS'; d := 'D'; n := 'N'; e := '.E';
  WriteLn(dt.Month, p, d, n, e);
end.
