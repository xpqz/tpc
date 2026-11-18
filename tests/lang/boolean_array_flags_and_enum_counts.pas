program FlagsCount;
type Suit = (Clubs, Diamonds, Hearts, Spades);
var seen: array[Suit] of Boolean; n: array[Suit] of Integer; i: Integer; s: Suit;
begin
  for s := Clubs to Spades do begin seen[s] := False; n[s] := 0 end;
  for i := 1 to 20 do begin
    s := Suit((i * 7) mod 3);
    seen[s] := True;
    n[s] := n[s] + 1
  end;
  for s := Clubs to Spades do WriteLn(Ord(s), ' ', seen[s], ' ', n[s])
end.
