{$IFDEF BIG}
const Size = 100;
{$IFDEF HUGE}
const Extra = 1;
{$ELSE}
const Extra = 2;
{$ENDIF}
{$ELSE}
const Size = 1;
{$ENDIF}
{$UNDEF BIG}
{$DEFINE SMALLER}
