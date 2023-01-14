use crate::{
    bigint::Bigint, block_encode::use_big_endian, encoding_declare, encoding_x_declare,
    use_padding, BlockEncoding,
};

encoding_declare!(Base2, prefix: '0' => "01", use_big_endian());
encoding_declare!(Base8, prefix: '7' => "01234567", use_big_endian());

encoding_x_declare!(Base10, prefix: '9' => "0123456789");

encoding_declare!(Base16, prefix: 'f' => "0123456789abcdef", use_big_endian());
encoding_declare!(Base16Upper, prefix: 'F' => "0123456789ABCDEF", use_big_endian());

encoding_declare!(Base32Hex, prefix: 'v' => "0123456789abcdefghijklmnopqrstuv", use_big_endian());
encoding_declare!(Base32HexUpper, prefix: 'V' => "0123456789ABCDEFGHIJKLMNOPQRSTUV", use_big_endian());
encoding_declare!(Base32HexPad, prefix: 't' => "0123456789abcdefghijklmnopqrstuv", use_big_endian(), use_padding('='));
encoding_declare!(Base32HexPadUpper, prefix: 'T' => "0123456789ABCDEFGHIJKLMNOPQRSTUV", use_big_endian(), use_padding('='));

encoding_declare!(Base32, prefix: 'b' => "abcdefghijklmnopqrstuvwxyz234567", use_big_endian());
encoding_declare!(Base32Upper, prefix: 'B' => "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", use_big_endian());
encoding_declare!(Base32Pad, prefix: 'c' => "abcdefghijklmnopqrstuvwxyz234567", use_big_endian(), use_padding('='));
encoding_declare!(Base32PadUpper, prefix: 'C' => "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", use_big_endian(), use_padding('='));

encoding_declare!(Base32Z, prefix: 'z' => "ybndrfg8ejkmcpqxot1uwisza345h769", use_big_endian());

encoding_x_declare!(Base36, prefix: 'k' => "0123456789abcdefghijklmnopqrstuvwxyz");
encoding_x_declare!(Base36Upper, prefix: 'K' => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ");

encoding_x_declare!(Base58Bitcoin, prefix: 'z' => "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
encoding_x_declare!(Base58Flickr, prefix: 'Z' => "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ");

encoding_declare!(Base64, prefix: 'm' => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", use_big_endian());
encoding_declare!(Base64Pad, prefix: 'M' => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", use_big_endian(), use_padding('='));
encoding_declare!(Base64URL, prefix: 'u' => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", use_big_endian());
encoding_declare!(Base64URLPad, prefix: 'U' => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", use_big_endian(), use_padding('='));

encoding_declare!(Base256Emoji, prefix: '🚀' => "🚀🪐☄🛰🌌🌑🌒🌓🌔🌕🌖🌗🌘🌍🌏🌎🐉☀💻🖥💾💿😂❤😍🤣😊🙏💕😭😘👍😅👏😁🔥🥰💔💖💙😢🤔😆🙄💪😉☺👌🤗💜😔😎😇🌹🤦🎉💞✌✨🤷😱😌🌸🙌😋💗💚😏💛🙂💓🤩😄😀🖤😃💯🙈👇🎶😒🤭❣😜💋👀😪😑💥🙋😞😩😡🤪👊🥳😥🤤👉💃😳✋😚😝😴🌟😬🙃🍀🌷😻😓⭐✅🥺🌈😈🤘💦✔😣🏃💐☹🎊💘😠☝😕🌺🎂🌻😐🖕💝🙊😹🗣💫💀👑🎵🤞😛🔴😤🌼😫⚽🤙☕🏆🤫👈😮🙆🍻🍃🐶💁😲🌿🧡🎁⚡🌞🎈❌✊👋😰🤨😶🤝🚶💰🍓💢🤟🙁🚨💨🤬✈🎀🍺🤓😙💟🌱😖👶🥴▶➡❓💎💸⬇😨🌚🦋😷🕺⚠🙅😟😵👎🤲🤠🤧📌🔵💅🧐🐾🍒😗🤑🌊🤯🐷☎💧😯💆👆🎤🙇🍑❄🌴💣🐸💌📍🥀🤢👅💡💩👐📸👻🤐🤮🎼🥵🚩🍎🍊👼💍📣🥂", use_big_endian());
