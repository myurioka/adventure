use serde::Serialize;

/* <-- CONSTANT VALUE */
pub const TITLE: &str = "LITTLE RED RIDING HOOD";
pub const LAST_PAGE:usize = 15;
pub const DEFAULT_COLOR: &str = "rgba(0,128, 0)";
pub const GEMINI_API_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key=";


pub const TEXT_SPACE:usize = 12;

pub const TEXT_OPEN: &'static str = " 
1812\n
GRIMM’S FAIRY TALES\n
LITTLE RED RIDING HOOD\n
Jacob Ludwig Grimm and Wilhelm Carl Grimm\n
";

pub const TEXT_CHAPTER:[&str;9] = [
"",
"Once upon a time, there was a lovely little girl who was\n
loved by everybody, especially her grandmother who\n
absolutely adored her. One day, her grandmother gave\n
her a beautiful red velvet riding cloak. \n
It looked so wonderful on her, and she wore it all \n
the time,so everyone started calling her Little Red\n
Riding Hood.\n\n
  One day, her mother said, 'Little Red Riding Hood,\n
sweetie,I've packed some delicious cakes and a flask\n
of wine for you to take to Grandma. She's feeling a bit\n
under the wheather, and these treats will cheer her up.\n
Now, hurry along before it, and walk nicely,\n
don't run! You miaght trip and break the wine flask,\n
and then Grandma wouldn't get any. And when you go\n
into her room,\n
[_________________________],\n
instead of starting about you.\n
”I will be sure to take care”\n
said Little Red Riding Hood to\n
her mother, and game her\n
hand upon it.\n",
"Now the grandmother lived away in the wood, half an hour's\n
walk from the village; and when Little Reda Riding Hood\n
had reached the wood, she met the wolf;but as she did\n
not know what a bad sort of animal he was, she did not\n
feel frightened. “Good day, Little Red Riding Hood,”\n
said he. “Thank you kindly, wolf,” answered she.\n
“[_______________________], Little Red Riding Hood?”\n
“To my grandmother’s.” “What are you carrying under your\n
apron?” “Cakes and wine; we baked yesterday; and my\n
grandmother is very weak and ill, so they will do her\n
good, and strengthen her.“Where\n
does your grandmother live, Little\n
Red Riding Hood?”\n
“A quarter of an hour’s walk\n
from here; her house stands\n
beneath the three oak trees,\n
and you may know it by the\n
hazel bushes,”\n
said Little Red Riding Hood.",
"The wolf thought to himself, “That tender young thing\n
would be a delicious morsel, and would taste better than\n
the old one;I must manage somehow to get both of them.”\n
Then he walked by Little Red Riding Hood a little while,\n
and said, “Little Red Riding  Hood,just look at the pretty\n
flowers that are growing all round you; and I don’t think\n
you are listening to the song of the birds; you are posting\n
along just as if you were going to school, and it is so\n
delightful out here in the wood.” Little Red Riding Hood\n
glanced round her, and when she saw the sunbeams darting\n
here and there through thetrees, and lovely flowers\n
everywhere, she thought to herself, “If I were\n
to take a fresh nosegay to my\n
grandmother she would be very\n
pleased,and it is so early in\n
the day that I shall reach her\n
in plenty of time”;  and so\n
[________________________].\n 
And as she picked one she saw\n
a still prettier one a little\n
farther off, and so she went\n
farther and farther into the wood.",
"But the wolf went straight to the grandmother’s house and\n
knocked at the door. “Who is there?” cried the grandmother.\n
“Little Red Riding Hood,” he answered,“and I have brought \n
you some cake and wine. Please open the door.” “Lift the\n
latch,” criedthe grandmother;“I am too feeble to get up.”\n
So the wolf lifted thelatch, and the door flew open, and\n
he fell on the grandmother and ate her up without saying\n
one word. Then he drew on her clothes,put on her cap,\n
lay down in her bed, and drew the curtains.\n
Little Red Riding Hood was all this time running about\n
among the flowers, and when she had gathered as many\n
as she could hold, she remembered her grandmother,and\n
set off to go to her. She was surprised to find the door\n
standing open,and when she came inside she felt very I\n
feel, and I was so glad this morning to go to my strange,\n
and thought to herself, “Oh dear,how uncomfortablen I feel,\n
and I was so glad this morning to go to my grandmother!”\n
And when she said, “Good morning,” there was no answer.\n
Then she went up to the bed and\n
drew back the curtains;\n
there lay the grandmother\n
[________________________],\n
so that she looked very odd.",
"“O grandmother, what large ears you have!”\n
“The better to hear with.” “O grandmother, what great\n
eyes you have!” “The better to see with.” “O grandmother,\n
what large hands you have!” ”The better to take hold of\n
you with!. “But, grandmother, what a terrible large mouth\n
you have!” “The better to devour you!” And no sooner\n
had the wolf said it than he made one bound from the bed,\n
and swallowed up poor Little Red Riding Hood. Then the\n
wolf,having satisfied his hunger, lay down again in the\n
bed,went to sleep, and began to snore loudly.The huntsman\n
heard him as he was passing by the house,and thought,\n
“How the oldwoman snores- I had better see if there is\n
anything the matter withher.” Then he went into the room,\n
and walked up to the bed,and saw the wolf lying there.\n
“At last I find you, you old sinner! said he;\n
[_______________________.]”\n
And he made up his mind that the\n
wolf had swallowed the grandmother\n
whole and that she might yet be saved.\n
So he did not fire, but took a pair of\n
shears and began to slit up the wolfs\n
body.When he made a few snips Little\n
Red Riding Hood appeared and after\n
a few more snips she jumped out and cried,",
"“Oh dear, how frightened I have been!It is so dark inside the\n
wolf.” And then out came the oldgrandmother, still living and\n
breathing. But Little Red Riding Hood went and quickly fetched\n
some large stones,with which she filled the wolf’s body,\n
so that when he waked up,and was going to rush away,\n
the stones were so heavy that he sank down and fell dead.\n
They were all three very pleased.The huntsman took off the\n
wolf’s skin,and carried it home.The grandmother ate the cakes\n
,and drank the wine,and held up her head again,and Little\n
Red Riding Hood said to herself that [____________________]\n,
but would mind what her mother told her.It must also be\n
related how a few days afterwards,when Little Red Riding Hood\n
was again aking cakes to her grandmother,another wolf spoke\n
to her,and wanted to tempt her to leave the path;but\n
she was on her guard,and went\n
straight on her way,and told her\n
grandmother how that the wolf had\n
met her,and wished her good day,\n
but had looked so wicked about\n
the eyes that she thought if it\n
had not been on the high road he\n
would have devoured her.",
"Come,” said the grandmother, “we will shut the door, so\n
that he may not get in.” Soon after came the wolf\n
knocking at the door,and calling out, [_______________],\n
bringing you cakes.” But they remained still, and did not\n
open,the door. After that the wolf slunk by the house,\n
and got at last upon the roof to wait until Little Red\n
Riding Hood should return home in the evening; then he\n
meant to spring down upon her, and devour her in the\n
darkness. But the grandmother discovered his plot. Now\n
there stood before the house a great stone trough, and\n
the grandmother said to the child, “Little Red Riding\n
Hood, I was boiling sausages yesterday, so take the\n
bucket, and carry away the water they were boiled in,\n
and pour it into the trough.” And\n
Little Red Riding Hood did so until\n
the great trough was quite full. When\n
the smell of the sausages reached he\n
snuffed it up,and looked round,and\n
stretched out his neck so far that he\n
lost his balance and began to slip,and\n
he slipped down off the roof straight\n
into the great trough,and was drowned.\n
Then Little Red Riding Hood went\n
cheerfully home, and came to no harm.",
""
];

pub const TEXT_CHAPTER_TEXT_PLACEHOLDER:[&str;9] = [
"Enter Gemini API Key",
"おはようと言うのを忘れないでね",
"とても早くどこへ行くの？",
"彼女は花を探して森の中を走り回りました",
"帽子を深々とかぶって",
"私は長い間お前を探していた。",
"もう決して一人で森の中をうろつくようなことはしない",
"おばあさん、ドアを開けて。赤ずきんだよ",
""
];

// Type Definition Of Gemini API Payload

#[derive(Serialize)]
pub struct GeminiRequestPart {
    pub text: String,
}

#[derive(Serialize)]
pub struct GeminiRequestContent {
    pub parts: Vec<GeminiRequestPart>,
}
#[derive(Serialize)]
pub struct GeminiRequestBody {
    pub contents: Vec<GeminiRequestContent>,
}
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct GeminiResponseCandidatePart {
    pub text: String,
}
#[derive(Deserialize, Debug)]
pub struct GeminiResponseCandidateContent {
    pub parts: Vec<GeminiResponseCandidatePart>,
}
#[derive(Deserialize, Debug)]
pub struct GeminiResponseCandidate {
    pub content: GeminiResponseCandidateContent,
}
#[derive(Deserialize, Debug)]
pub struct GeminiResponseBody {
    pub candidates: Vec<GeminiResponseCandidate>,
}

// sanitize input text

pub fn sanitize(text: String) -> String{
    let mut chars:Vec<char> = Vec::new();
    for c in text.chars(){
        match c {
            '&' => continue,
            '<' => continue,
            '>' => continue,
            '"' => continue,
            '\\' => continue,
            '\'' => continue,
            '.' => continue,
            _ => chars.push(c),
        }
    }
    String::from_iter(chars.iter())
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageType {
    Input,
    Output,
    First,
    Fin,
}