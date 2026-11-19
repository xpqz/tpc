{ The Utils unit: routines that Turbo Pascal 5.0 lacks and programs keep writing for
  themselves. It is not part of TP 5.0 (docs/deviations.md); it is plain TP 5.0 Pascal, so the
  real compiler accepts this same file. A program that declares one of these names itself is
  unaffected: its own declaration wins. }
unit Utils;

interface

function Power(Base, Exponent: Real): Real;
function Tan(X: Real): Real;
function ArcSin(X: Real): Real;
function ArcCos(X: Real): Real;
function Log10(X: Real): Real;
function Hex(Value: LongInt; Digits: Byte): String;
function TrimLeft(S: String): String;
function TrimRight(S: String): String;
function Trim(S: String): String;
function UpperCase(S: String): String;
function LowerCase(S: String): String;
function Min(A, B: LongInt): LongInt;
function Max(A, B: LongInt): LongInt;
function LeadingZero(Value: LongInt; Width: Byte): String;

implementation

{ Base ^ Exponent = e ^ (Exponent * ln Base), which needs Base > 0. Exponent 0 gives 1 and
  Base 0 with a positive Exponent gives 0. A negative Base with a whole Exponent is the power
  of -Base, negated when Exponent is odd. Every other case (Base 0 with a negative Exponent,
  a negative Base with a fraction) reaches Ln of a non-positive number: runtime error 207. }
function Power(Base, Exponent: Real): Real;
begin
  if Exponent = 0 then
    Power := 1
  else if (Base = 0) and (Exponent > 0) then
    Power := 0
  else if (Base >= 0) or (Exponent <> Int(Exponent)) then
    Power := Exp(Exponent * Ln(Base))
  else if Frac(Exponent / 2) = 0 then
    Power := Power(-Base, Exponent)
  else
    Power := -Power(-Base, Exponent);
end;

function Tan(X: Real): Real;
begin
  Tan := Sin(X) / Cos(X);
end;

{ The angle whose sine is X, from ArcTan; X = 1 and -1 would divide by zero, so they are
  the two poles. For |X| > 1, TP's Sqrt fails with runtime error 207, so it is raised here. }
function ArcSin(X: Real): Real;
begin
  if Abs(X) > 1 then
    RunError(207);
  if Abs(X) = 1 then
    ArcSin := X * Pi / 2
  else
    ArcSin := ArcTan(X / Sqrt(1 - X * X));
end;

function ArcCos(X: Real): Real;
begin
  ArcCos := Pi / 2 - ArcSin(X);
end;

function Log10(X: Real): Real;
begin
  Log10 := Ln(X) / Ln(10);
end;

{ One hex digit per pass, lowest nibble first. Shr is a logical shift, so a negative Value
  comes out in two's complement, cut to the last Digits nibbles; a Value that needs more
  digits than Digits loses its high ones. }
function Hex(Value: LongInt; Digits: Byte): String;
const
  HexDigits: String = '0123456789ABCDEF';
var
  Text: String;
  I: Integer;
begin
  Text := '';
  for I := 1 to Digits do
  begin
    Text := HexDigits[(Value and 15) + 1] + Text;
    Value := Value shr 4;
  end;
  Hex := Text;
end;

{ Blanks and control characters (everything up to ' ') are trimmed. }
function TrimLeft(S: String): String;
var
  First: Integer;
begin
  First := 1;
  while (First <= Length(S)) and (S[First] <= ' ') do Inc(First);
  TrimLeft := Copy(S, First, 255);
end;

function TrimRight(S: String): String;
var
  Last: Integer;
begin
  Last := Length(S);
  while (Last > 0) and (S[Last] <= ' ') do Dec(Last);
  TrimRight := Copy(S, 1, Last);
end;

function Trim(S: String): String;
begin
  Trim := TrimLeft(TrimRight(S));
end;

{ ASCII letters only; every other character, including those above 127, is unchanged. }
function UpperCase(S: String): String;
var
  I: Integer;
begin
  for I := 1 to Length(S) do
    if S[I] in ['a'..'z'] then S[I] := Chr(Ord(S[I]) - 32);
  UpperCase := S;
end;

function LowerCase(S: String): String;
var
  I: Integer;
begin
  for I := 1 to Length(S) do
    if S[I] in ['A'..'Z'] then S[I] := Chr(Ord(S[I]) + 32);
  LowerCase := S;
end;

function Min(A, B: LongInt): LongInt;
begin
  if A < B then Min := A else Min := B;
end;

function Max(A, B: LongInt): LongInt;
begin
  if A > B then Max := A else Max := B;
end;

{ Value in decimal, padded with zeros after the sign up to Width characters (the sign
  counts). A Width smaller than the number leaves it whole: 12345 with Width 3 is '12345'. }
function LeadingZero(Value: LongInt; Width: Byte): String;
var
  Sign, Digits: String;
begin
  Str(Value, Digits);
  Sign := '';
  if Digits[1] = '-' then
  begin
    Sign := '-';
    Delete(Digits, 1, 1);
  end;
  while Length(Sign) + Length(Digits) < Width do Insert('0', Digits, 1);
  LeadingZero := Sign + Digits;
end;

end.
