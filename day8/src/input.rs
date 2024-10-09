use std::string::String;

pub fn puzzle_input() -> String {
    "220212222222222222222222212222222222222222220222022222021202222122212222120202222222222222220222022022222222202202212022222200222222222202222212212022222222222222222222222222202222222222222222221222022222122212222022202222020222222222222222220222222222222222222202212022222210222222222202222212202022222222222222222222222222212222222222222222221222122222020212222122222222121212222222222222220222022022222222222212212022222202222222222202222202212022221202222222222222222222222222222222222222220220222222221222222022202222222202222222222222220221122122222222222222212222222201222222222202222202202022220202222222222222222222222222222222222222222222022222120212222122212222021202222222222222222221022022222222222212202122222221222222222202222212202122222222222222222222222222212222222222222222220220022222120202222222212222120222222222222222221222222222222222222212222222222202222222222222222212202122220202222222222222222222202222222222222222222222222222122202222022222221022202222222222222221220222122222222201212222022222201222222222202222202202222222202222222222222222222212222222222222222222220222222221212222122202220221222222222222222222221122022222222200202222022222201222222222212222212222022221222222222222222222212212222222222222222222222022222222222222122212221121222222222222222221220122122222222210202212122222211222222222222222222212222221212222222222222222202202222222222222222222221222222222202222122212220020212222222222222221222022222222222212202202022222200222222222202222202222122221202222222222222222212202222222222222222221222022222121222222022222221220202222222222222221222022222222222211202202122222211222212222202222112222022222202222222222222222222212222222222222222222222022222021222222022202222020222222222222222221221122122222222210212222222222201022202222202222122202222222222222222222222222212212222202222222222220221222222020202222222202222121212222222222222220221122022222222221202202222222212222202222212222012222022220202222222222122222212222222212222222222221221022222220212222022212222221212221222222222222222122122222222201202202222222201222222222212222112202222222202222222222122222202202222212222222222222221022222021202221222202222221202220222222222222222122022202222201212202122222222222212222212222202202122220222222222222222222212212222220222222222220222022222222212221022222222222212222222222222220222122222212222200212202022222211222222222222222102212222220222222222222122222202212222202222222222222222022222021202221222202220022222220222222222220220022222222222222222222122222212222202222202222222222222221222222222222022222212222222222222222222220221022222120212222222222220122202221222222222221220222122212222200222212120222221122202222222222012222222221202222222222022222222222222202222222222220222022222021222221122202222121202222222222222220222222122222222200212212222222212222202222222222222222022220202222222222022222222212222211222222222222200022222121202221122222222221222020222222222201220022122222222211222202021222211222222222202222222212022222202222222222122222212202222212222222222221202122222121222220022212220121212222222222222212221022022202222222212212020222221022202222212222102222022220202222222222122222202222222202222222222222200022222222202222122222221022212022222222222121220022022202222221212222220222212222202222222222202222122220202222222222122222202202222222222222222220210122222020212220222212221020202121220222222011222022022202222202222212122222211022212222222222002212122202222222222222222222202212222200222222222220212022222122212222022202222120222120221222222110222222122212222211212222021202212122222222212222122202222221222222222220222222202222222200222222222222201222222122202212022202222121222120220222222212220122122212222221212202121202201022222222212222102212022210222222222220022222202212222201222222222220201222222121222211122202220022222221222222222201221122022222222201212202120222222222222222212222022212222210202222222222022222212222222201222222222222201122222211202220022202220121212121222222222110220222122202222221222012120202201222212222222222222012022201212222222220222222222222222212222222222220221222222120222221222222221222202222221222222111222122122212222220222212022202010022212222212222212122222212212222222221022222212212222212222222222220220222222111212212022202220221211022222222222111222222222202222200212122221222011022222222212222022212122200222222222221122222222212222211222222222222200022222100212210022212221100222220220222222222221122122222222211222112120222122022202222222222122222022211202222222221122222222222222210222222222220201222222110212221022202221120220220221022222102220222022222222210202002122212112122222222222222022022120211212222222220122222202212222222222222022222220122222110202220122212222201210122222022222000220122022222222220222122220222002222212222222222102110020211202222222221122222202202222220222222022222211022222112202222022212222102211120222122222011220220122222220202202022221222201022222222222222002020121212202222222220022222202222222202222222122222221122222010202220122212220011210122220022222222221020222202222201222202220222012222202222212222112222122222212222222222222222222222222221222222222222222022222010212210222212222000222121222022222212222220122222222212222022022202101022212222222222202120022221222222222221122222222202222202222222122221202222222001222202222202220110021020221222222002222020222212221211212222222212110022212222202222202212022210202222222220122222222212222200222222022221202222222220222211122202220220120221221222222010222121122202222221212222120222010022022222212222122102122221212222222221022222222212222222222202022222200022222222212202022212221210112121220022222220222221122202222202212012021202211122222222222222202000022211222222222221022222212202222220222222122221201122222211222220022222221022022221220022222111220022022202221221222012121222211022212222202222202112022222222222222222222222222212222200222202022220202122222021202221222212221010111220221022222001221121122220222222222122022222211222202222212222102220120212212222222221222222202222222221222202122220201022222200222221222222220020021122220122222011220020022200220221212002020222122222212222212222102012020221212222222221222222202222222212222212122222202222202010212200222202222122100021220222222221221220022220222202202012220202212220212222212222212211222211222222222222122222202222222202222212022220212222112111202202222222222121011221220122222100222022022201220202212212020222222020002222202222222100020222222212222221222222212222222211222202022222222122012220212211022212221210111220222222222020220020222202221201222212021222100120102222212222012202022201212222222220122221212212222200222202122220220222022020212211122202221012002220222122222002221020022211221211222202222222222220122222220222102000122211202222222222122222222012222212222212122220200022122012222201222122220222010222222222222222220020222202221221212102020222012121222222220222122020122221222202222221022220222202222201222202022222200122022021202210122122222000100121222022222101220222220212222201202112122202212022112222222222212112020200222212222221122222202112222200222222022221211122212102202221222212221110012122222022222100221220221210221202212202121212222122222222220222222202020222212202222220222220222122222212222201022222211022202122222211022102221202001222220122222122221120020202220211212202121222002122202222220222202012221202211212222221022222212122222210222201222221220022112002212210222212221102112221220022222101222222221222220201202202120222212022012222220222012120122220210202222220122222212012222210222201222221202122102111202210022112222202011220222222222210222222021200222200222002120202100220222222211222002001121211212202222221122222212112222201222201222220200222012221202211222122220212000122220022222111222120122221222202222202202212221020202222222222212212221211202222222222022221212202222201222210122221212122202120202202122102222111200121222122222022220120221212222201202012002222221220212222221222202021020211212202222220122222222222022220202220022221202122012102222212222002221221022021221022222220220220122200221222202020110210120121002222202222222110222221212222222220121222212022122210222212122220211222212012222212222202220002021221221122222221222022122200220220212112221200020021022222221222202120022222210212222222020222202212122220212210122221211222112110202200122112220112012222220222222100221221221200220210222112101211001020202222202222211022021212211222222220121221212222122221212201022221212222222210202200022022222120220120221122222212220220021200222221202020122211220020012222211222111021021220212202222221020220222122022200202212022222210022012002222210022122221002202021222122222000220220021200120221212220112210010021222222202222122102121212211222222222220222222102122211202221222220220222112000212220122102222101210122220222222022221122122221200210222022022201001021212222221222201002121210200202222220221220212002021221212222122221200122122200222212022202220222122120222220222102222220120221112212001112100220222120012222201222121022221202210222222220120222212202121211222211222222221222112212222210022112221002111221221021222201221022220200001211210002202220020122002222212222021200222200220202222220020222212012121200202221122221210022002201202210222002222020210021221022022012220120022202021202220002021211001121222222211222102200121221222222222222122222202222120200212210222220200122202221222211022000220020122021222121022200220220222211101200120001212200002220202222221222022122020201221222222221220221212002022221202200222221212222002110212210122000220120110122221122022002222022220201222221220012200202111120122222202222022111022222212202222220220220212122021221222210022221210122212011212202022020222100112221220221122000220221221210220202021210102211211221222222222222100212122201210212222220021222212112020210212211022222220022002001212121122112220221102121222020222120220221020212022220120220001201211121002222222222211101021211200212222220020220212102122211212220222220212122212000202201222011221020011021221021122111222220021220020202102012121102202210011212222222202120222211202222222220121220222012222202212200222220202222102021202100202022220022202021222022122220221122222211220221020211100011221122201212212222002111120222212222222221211222222012201212212222022222212122022200202112002112220202111022222120222022221122021201022222022120001101011021121202212221111102020221211202222222102220202112102211212220222222221022102011202121222212220200110220222021122222220122220211102210112012022021211100101202202220021212222210200222222221110221222222020210222222122222220222102011222220212122222020110120222221020100222022121201120212211020102220211111120202210221002011021211220202222222111220212022021221202212222220210122212200202200102220222000102220201221120221222220222201202210011000111012002220222222211221220112121210220202222222121221222012100220212200122220200022222222212102202010220010021220222121121210221020120212212211111220000120121111120202202221222012020220210212222221110222212202022220202222222221211222102101222222022002221021210221222121021002022120121220111211120211001110212121110212221220201000020210220212222220101221202022011222202210122222222122122120212122222220220210210221212021120002221221121221002202112122112011202120121112210222011002120202202202222220001220202102022210202222022221212022212122222220122212221122201021221210022220122122020212011200111210110022110102001012200122210122121212200212222222012221202022111201202000222220211222222012202210112022222201220221202000221210021120122210012220200200201202111210122122212022000220022202221212222220121221212202120222222110122220202022212011202001101021222221112221202021222121122221122210022200021011002210021120020122202020111200020221222202222220220221222022010222222000222221212022022101202020222200222212220121200122022002121021221221122220002110021000022021010022210020012011220220212212222221202220212102000211202100122221110122002220212210212222221120000121202211021111221022122210100212022121021210022021000002220222212200220200200212222221012220212002201200202112122220021122102120212210022101220002112022222011020122101220022211221211202022212102100020211212210022121102120221201212222221221222212122000201202202122221110122012022212210020101221001012122221101020222120022121201101202020202121001120202102202222021012211221202222202222220012222202002201210212022122200122122212120222210212010221222220020211221121111012022222210202200201221002101220222211122201221210112122221212212222220002221202222101221222110022202000122012200212020001010222002110222222201020210020022220221111220000100102011112110212002200201021011122201202202222221110220202212101222212021122201011122112020212120000221221000022220201101022022101220120220100202021220220222221200211222200010200112022201200222222221100222212012122211222002022211102122202202202100020022220101221021201211022110220022221211101202221220221101210201020002211100221110020200221202222222112220202012110220212001222222200222222220212100001100221201201220212122212212200220021210101201221200010212021121212212222210202220122220200222222222102222222022210202212100122211200022222122222201112210221212110121212221122200202121121211110202021211200200100100112022210011212111122220202222222222210220212112020200202202022210012022012122202220202221222202000120210101112111022220020210022200011212221102120102112112202121200010122222210202222221110221212102020221212202222202010022002021212112202200220022020001211112222000020021220201000222012002011202222211020212221111112121221212222212222221022221202202201220222211122201122022012020202001120202220122222100202002022100020021021202202222102100221220020221122102220212002221022212210202222220200221202002201210222210222221110122222020222012111120222001002211201220202220022021202202121202011020102222111002220002200101222222022221211202222222021222202222020220202101122200111222112011222001120112222122020102202002002120201122011212102222001110010102122212020012200111100122121212222202222221010221212222110222202001220200011022002220202111111022221102202000211200212012122120111210201222121120022110222020211102220120020222021211222222222222200221202222020212212210021211120022102212212212100200220210010120201010012011220220011222000212011111222210122102000212222201010001222200220212222220200222222102002201212211120211002122202221222010011110222120122211200211022212211122021222202201021022222200000202021222221010120011021221221212222221001120102202122211212121022211101022202211212221111211221112221001200102102010001220120200022222012111120212222111001012221221201011122220020122220222100221212222011222212110220212202222102121212021222112222110210212220201220110212022002220111201210212100001000220021122210222120011102212000202222220212222002022221211012002020212012022222011222112111220220101121021212101210120202121000222022222020221101120001010220112210111102021002122021011011010100100221020102101011222212012120200121111100111120001011210100101012201111001110201020101122001011000001200120101101120121021100210212".to_string()
}
