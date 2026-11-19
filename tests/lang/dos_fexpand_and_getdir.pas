program DosFExpandAndGetDir;
uses Dos;
var
  s: string;

procedure T(p: string);
begin
  WriteLn('[', p, '] -> [', FExpand(p), ']');
end;

begin
  GetDir(0, s); WriteLn(s);
  GetDir(1, s); WriteLn(s);
  GetDir(3, s); WriteLn(s);
  GetDir(4, s); WriteLn(s);
  T('a.txt'); T('A.Txt'); T('\x\y.z'); T('c:\q'); T('d:f'); T('');
  T('.\a'); T('..\a'); T('x\..\y'); T('\a\.\b'); T('\..\a'); T('c:'); T('\');
  T('a\'); T('x\y\..'); T('*.*'); T('a b'); T('c:\a\b\..\..\..\z');
  T('C:\\a'); T('con'); T('a.b.c'); T('.'); T('..'); T('d:');
  T('a\.\'); T('a\..\'); T('\\a'); T('a\\b'); T('..\..\'); T('a\.');
  T('c:..\x'); T('1:x'); T('ab:x'); T('c:\a\..'); T('\a\..\..\b\'); T('x/y');
end.
