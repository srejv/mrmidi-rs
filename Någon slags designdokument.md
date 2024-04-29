Någon slags designdokument

Jag tänker:

* Digi* någon som kontroller. Antagligen Syntakt. :shrug: Digitone kanske e bra. Digitakt kanske e bra.
* Två spår med var sin scenegraf.
* Ett PostFX spår, som även fungerar som mixer mellan de två spåren.
* Ha en AppState eller något som håller i allt. 
* Hooka på en editor som låter en ändra på datastrukturen för renderingen;
 - Vad som finns i de två renderingsträden
  > Gif, position, opactiy?, transparency?, rotation?, scale?
  > Sprite, position, etc, SpriteAtlas to fit more into 1? Maybe you only get 1.
  > 3D-model, animationer?
 - Animationskurvor
  > Tweens
  > LFO
  > Designer, välj punkter, interpolationssätt(tween) mellan, loop eller ej, start phase?
* EventManager
 - Transformerar events till datastruktursförändringar
  > Add
  > Delete
  > Change Component (Variable?)
  > Has Component
  > Get Component
* MIDI input
 - Konvertera till EventManager Actions. 
  > Tabell med midievent -> EventManager::Event enum
  > Spara i rocks.db? Kanske där man ska ha sin data?
  > När ett MIDI event kommer in, skapa en action som tar den grejen
  > (ECS: Kanske ett debugsystem som bara loggar all input?)
* Projektstrukturen måste vara serializable. Just nu är det kaos. (Se test-setup.xml?)

Track1 --v 
Track2 ->==>PostFx -> Renderad Bild

State
 Tracks
  - Camera
  - List of Renderables
  - PostFX?

 PostFX
  - List of PostFX
   > One input is Tracks rendertextures
   > you get X slots? 
   > possible to change order? would be nice
   > add custom ones while running? maybe.
   > have some defaults like A, B, A over B, B over A?
* OSC integration? IDK.

Possible editor pages

Track Page 
 - Viewport Page?

3D Camera Page
 - Position (x,y,z)
 - Rotation (x,y,z)
 - FieldOfView

2D Camera Page
 - Position (x,y)
 - Rotation (z)
 - FieldOfView/Size

Transform Page
 - Position Page
 - Rotation Page
 - Visible Page?

Color Page
 - Transparency
 - Opacity?
 - Tint (Color)

Modulation Page(s)
 - ADSR, LFO, Designer Etc
 - Destination1, Depth1, Destination 2, Depth2

