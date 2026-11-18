program ProcVarResultTypes;
{$F+}
{ procedural functions of each scalar result type }
type
  TB = function(x: Integer): Byte;
  TS = function(x: Integer): ShortInt;
  TI = function(x: Integer): Integer;
  TW = function(x: Integer): Word;
  TL = function(x: Integer): LongInt;
  TT = function(x: Integer): Boolean;
  TC = function(x: Integer): Char;
function FB(x: Integer): Byte; begin FB := x end;
function FS(x: Integer): ShortInt; begin FS := x end;
function FI(x: Integer): Integer; begin FI := x * 2 end;
function FW(x: Integer): Word; begin FW := x end;
function FL(x: Integer): LongInt; begin FL := x * 100000 end;
function FT(x: Integer): Boolean; begin FT := x > 0 end;
function FC(x: Integer): Char; begin FC := Chr(x) end;
var b: TB; s: TS; i: TI; w: TW; l: TL; t: TT; c: TC;
begin
  b := FB; s := FS; i := FI; w := FW; l := FL; t := FT; c := FC;
  WriteLn(b(300), ' ', s(200), ' ', i(20000), ' ', w(-1), ' ', l(30000), ' ', t(-1), ' ', t(1), ' ', c(66))
end.
