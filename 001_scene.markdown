# Scene files

## Procedural vs. declarative

PBR distinguishes between procedural scene description formats and
declarative ones. The pros and cons are the classic ones; procedural
gives greater specific control over the rendering mechanisms while being
coupled to implementation semantics, while declarative gives up that
control in favour of a more general specification format that can be
used in different ways.

Declarative: good innit. Certainly for pre-composed scenes like ours.

## Immediate vs Retained mode

A further tradeoff is whether to essentially pin an object's properties
when they're declared, or to allow later modifying statements.

Immediate mode gives several assumptions that can be useful for high
performance interactive graphics (because the renderer can start
optimising the scene without worrying that it'll later change). These
may not apply to pre-composed rendering like ours.

PBR goes with the former nonetheless.
