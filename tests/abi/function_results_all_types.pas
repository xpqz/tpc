program FunctionResultsAllTypes;
{ functions of each scalar result type: return paths, wrap on return }
function FB(x: Integer): Byte; begin FB := x end;
function FS(x: Integer): ShortInt; begin FS := x end;
function FI(x: LongInt): Integer; begin FI := x end;
function FW(x: LongInt): Word; begin FW := x end;
function FL(x: Integer): LongInt; begin FL := x * 1000 end;
function FT(x: Integer): Boolean; begin FT := x > 0 end;
function FC(x: Integer): Char; begin FC := Chr(x) end;
begin
  WriteLn(FB(300), ' ', FB(-1), ' ', FS(200), ' ', FS(-129), ' ', FI(70000), ' ', FI(-40000));
  WriteLn(FW(-1), ' ', FW(70000), ' ', FL(32000), ' ', FL(-5), ' ', FT(0), ' ', FT(3), ' ', FC(97))
end.
