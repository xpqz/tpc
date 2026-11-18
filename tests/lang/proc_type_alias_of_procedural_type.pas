program ProcTypeAliasOfProceduralType;
{$F+}
{ a type name for a procedural type is the same type; variables of the alias and of the
  original are interchangeable, also as parameters and var parameters }
type TF = function(x: Integer): Integer;
type TG = TF;
type TH = TG;
var f: TF; g: TG; h: TH;
function Inc1(x: Integer): Integer; begin Inc1 := x + 1 end;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
procedure Apply(p: TH; var out: TG; n: Integer);
begin
  out := p;
  WriteLn(p(n), ' ', out(n))
end;
begin
  f := Inc1; g := f; h := g;
  WriteLn(f(1), ' ', g(2), ' ', h(3));
  Apply(Dbl, f, 10);
  Apply(f, g, 7);
  WriteLn(f(4), ' ', g(4))
end.
