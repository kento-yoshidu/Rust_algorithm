// https://atcoder.jp/contests/abc113/tasks/abc113_b

use itertools::Itertools;

pub fn run(n: i32, t: i32, a: i32, vec: Vec<i32>) -> usize {
    let mut tmp: Vec<i32> = Vec::new();

    for i in 0..n {
        let num = t as f64 - vec[i as usize] as f64 * 0.006;

        tmp.push((a - num as i32).abs())
    }

    tmp.iter().position_min().unwrap() + 1
}

pub fn run2(_n: i32, t: i32, a: i32, vec: Vec<i32>) -> usize {
    let v: Vec<f32> = vec.iter().map(|n| *n as f32).collect();

    v.iter()
        .enumerate()
        .map(|(index, i)| {
            let tmp =  t as f32 - *i * 0.006;

            (index, (a as f32 - tmp).abs())
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, _)| i )
        .unwrap() + 1
}

/*
pub fn run3(_n: i32, t: i32, a: i32, vec: Vec<i32>) -> usize {
    let v: Vec<f32> = vec.iter().map(|n| *n as f32).collect();

    v.iter()
        .enumerate()
        .fold((0, std::f32::MAX), |(index, state), (i, num)| {
            let tmp =  t as f32 - num * 0.006;

            if (a as f32 - tmp).abs() < state {
                println!("{}", a as f32 - tmp);
                (i, (a as f32 - tmp))
            } else {
                (index, state)
            }
        }).0 + 1
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(2, 12, 5, vec![1000, 2000]));
        assert_eq!(3, run(3, 21, -11, vec![81234, 94124, 52141, 2000]));
        // assert_eq!(449, run(1000, 33, -60, vec![104, 78360, 32508, 56373, 76117, 50118, 1899, 59880, 44803, 76957, 49649, 40521, 47071, 89211, 9964, 57730, 46448, 10519, 10652, 16973, 12841, 90415, 33163, 60513, 11391, 87986, 64119, 70472, 76919, 44026, 17074, 83943, 62647, 57583, 48199, 86982, 44138, 53726, 35907, 48086, 37048, 78646, 15737, 66240, 19095, 68407, 69424, 99647, 84806, 38466, 97408, 54831, 38963, 77248, 48049, 34077, 41360, 52187, 54590, 70802, 13519, 48855, 82349, 72516, 74310, 7246, 47976, 5411, 52026, 16367, 77126, 63478, 77383, 26682, 35248, 9999, 23963, 2414, 94652, 40659, 79738, 23356, 93374, 21177, 39803, 55618, 51244, 85491, 96217, 51276, 71113, 86731, 53704, 54864, 95347, 94088, 18448, 90171, 48970, 1273, 23760, 95984, 44604, 76659, 81689, 43989, 75208, 72914, 86445, 91897, 10289, 65413, 95450, 87315, 50434, 30753, 78000, 37316, 56687, 61115, 94985, 2225, 77283, 67318, 63754, 83479, 81160, 12781, 55077, 33074, 37400, 97454, 70833, 53830, 46809, 25356, 84780, 18067, 95762, 39819, 83936, 47013, 1158, 75540, 6366, 81675, 26628, 14904, 26620, 54944, 53415, 13475, 44746, 8669, 1320, 39553, 11511, 96908, 87221, 20387, 41216, 59239, 33526, 60296, 17671, 36476, 6002, 86369, 55804, 18116, 32159, 19918, 90262, 80429, 18475, 63806, 68252, 18739, 14203, 5382, 34098, 73674, 51296, 26838, 66570, 67705, 6396, 92204, 21566, 46409, 26705, 92688, 91122, 31118, 30887, 73070, 54452, 67255, 11332, 88577, 61131, 81781, 96840, 21825, 66019, 50635, 71042, 69803, 84529, 79693, 59074, 5301, 49715, 81683, 8873, 44121, 32126, 5190, 28403, 37227, 79493, 25147, 89476, 84047, 54090, 62523, 32050, 36702, 69795, 15550, 96348, 1164, 56659, 23842, 62915, 62708, 85607, 48357, 13217, 87372, 41693, 85584, 98439, 6132, 96876, 36934, 62978, 99750, 58103, 31762, 43242, 5528, 75292, 27863, 1313, 11756, 59118, 4400, 83966, 13440, 56873, 27868, 67392, 93813, 523, 26, 5927, 76977, 92949, 98702, 49280, 97101, 74439, 71377, 97441, 44983, 59064, 31855, 64661, 19391, 64423, 27938, 3350, 27498, 71023, 97832, 11527, 54245, 91176, 51001, 81286, 51474, 11930, 19325, 97433, 67801, 58038, 77475, 50262, 9578, 56410, 25801, 51561, 61328, 61156, 41296, 96231, 10666, 57105, 51557, 70102, 28254, 5908, 26726, 22359, 33553, 45256, 20096, 76250, 57663, 28249, 15618, 15033, 46508, 75328, 85142, 92299, 97724, 46724, 12579, 96457, 17938, 36365, 74400, 82839, 55620, 23293, 41774, 60120, 47813, 81964, 79702, 4423, 20202, 58692, 14164, 84498, 20170, 51248, 67569, 36764, 32949, 80346, 9329, 81437, 69849, 94387, 41491, 67880, 76125, 89519, 30261, 63960, 45862, 52797, 63312, 16956, 91049, 24964, 26850, 46314, 95185, 18619, 82943, 95297, 75001, 60837, 95609, 52791, 85352, 4643, 27445, 21688, 2123, 99974, 11255, 61396, 60063, 81111, 39180, 70154, 80539, 25635, 63984, 4424, 67129, 43116, 50112, 6607, 9927, 76537, 98156, 64817, 65913, 3729, 44673, 79176, 2581, 58621, 24993, 62505, 36004, 16556, 1131, 54426, 91955, 66806, 9342, 16691, 16552, 91592, 99217, 90390, 781, 74778, 55948, 90113, 16844, 16302, 522, 17706, 4584, 19826, 17246, 28200, 34823, 79795, 907, 837, 51519, 38389, 23559, 76322, 25970, 20043, 73532, 17327, 76138, 15453, 25039, 46221, 86019, 38741, 29728, 32359, 62273, 74129, 93265, 85814, 16009, 12574, 34410, 14372, 65305, 47781, 85711, 36011, 28158, 59039, 86673, 23828, 30199, 34636, 42158, 55674, 55829, 35247, 50354, 64299, 43901, 25053, 38195, 48651, 37013, 91010, 81028, 71980, 6983, 42620, 87288, 43603, 99795, 12165, 13129, 37429, 60730, 29663, 69655, 52619, 59839, 95393, 33421, 67352, 83012, 98956, 58237, 56391, 72570, 45038, 83134, 82339, 83590, 1039, 84655, 99770, 81112, 54989, 47686, 82299, 67522, 74152, 60331, 71182, 10861, 64762, 56583, 83412, 32116, 36756, 27522, 72897, 61837, 24764, 38049, 52114, 98947, 74843, 53103, 40259, 48332, 31971, 86765, 37454, 91542, 72502, 61011, 26540, 94054, 70064, 70493, 10193, 58313, 51202, 79320, 28202, 44818, 96622, 47251, 69198, 64021, 26336, 93899, 63374, 95790, 26220, 74870, 87231, 52882, 62712, 87628, 53340, 74488, 77092, 79398, 74595, 7797, 70340, 56746, 78158, 8614, 7006, 10422, 33897, 47079, 78680, 67677, 58287, 64695, 95640, 65188, 58334, 25899, 91481, 65490, 90798, 2805, 81396, 71453, 4232, 74407, 22834, 6535, 4993, 92670, 33852, 35710, 38022, 13021, 86526, 87230, 90757, 6163, 30479, 25468, 98735, 27224, 41741, 47475, 32977, 3437, 94269, 29085, 22522, 22425, 36894, 58747, 19806, 42682, 11758, 16702, 32590, 583, 11764, 8581, 3763, 51652, 70053, 45170, 45448, 70894, 13140, 57259, 32577, 88684, 59772, 19032, 97221, 54705, 80126, 64582, 13127, 61591, 48105, 87411, 54903, 15571, 44104, 8397, 24438, 32624, 38656, 6974, 74570, 97227, 41794, 6020, 79855, 35643, 69462, 69333, 69433, 70500, 42761, 99963, 44589, 28309, 76660, 55893, 71468, 41874, 78814, 93390, 53535, 46327, 33136, 83871, 12883, 63345, 94288, 51612, 58530, 57662, 55351, 18689, 47469, 59241, 45422, 23630, 8373, 11076, 2057, 3760, 8026, 36435, 61079, 17096, 86321, 66646, 98422, 67976, 54845, 64156, 747, 88792, 89201, 3486, 90639, 77041, 35449, 14483, 94399, 91604, 84560, 13628, 13254, 47060, 82745, 91848, 36152, 86779, 81335, 51521, 36348, 95962, 66158, 46982, 22375, 66798, 35307, 90568, 20545, 79047, 84293, 58443, 62616, 65581, 24029, 14602, 21854, 92195, 52843, 91105, 18181, 96392, 43810, 33209, 16110, 82010, 62229, 93093, 38666, 65877, 90679, 55529, 22295, 32804, 85590, 97475, 59853, 36630, 28835, 83121, 23167, 94272, 11, 84027, 41066, 24380, 62896, 57615, 30812, 8510, 30060, 48965, 54834, 15667, 43216, 61555, 64901, 9913, 60737, 18344, 9941, 80263, 70020, 66465, 54352, 65192, 68069, 47118, 37422, 75123, 90177, 94817, 70601, 85362, 61248, 76797, 73939, 86624, 86234, 23791, 60332, 96620, 35996, 26078, 25066, 72893, 51547, 57101, 96678, 83085, 33599, 32204, 14890, 64189, 70137, 9663, 70226, 49418, 86224, 49854, 51376, 99275, 84610, 88138, 88993, 81865, 21482, 89874, 96118, 34538, 74323, 95855, 88687, 31060, 66072, 92993, 90861, 41166, 32059, 3791, 45992, 84052, 15314, 80493, 5111, 51526, 43178, 82272, 84262, 68099, 58073, 84871, 83020, 47061, 38242, 5725, 19624, 69539, 64402, 10046, 80220, 70891, 99246, 25188, 37490, 77067, 36646, 49273, 20538, 35833, 43636, 13841, 48611, 21841, 84579, 46350, 47332, 4416, 11377, 76446, 41884, 96130, 29299, 34609, 83446, 5223, 36456, 26949, 74975, 29776, 7146, 44879, 45112, 10566, 74318, 54553, 63333, 93877, 84684, 13173, 15107, 144, 69453, 90399, 6328, 53669, 88546, 53866, 65405, 43016, 66246, 82942, 14898, 9972, 49721, 78138, 58566, 34290, 61851, 79393, 22641, 60297, 34620, 38166, 26930, 92148, 94779, 74968, 57436, 92985, 70574, 47928, 18316, 90285, 11010, 45115, 87700, 97215, 85809, 50469, 84306, 39674, 56279, 96077, 76014, 180, 15251, 47146, 86795, 17594, 51232, 43958, 71749, 90413, 55329, 33855, 56606, 80324, 84816, 69970, 50925, 17099, 36064, 82328, 63715, 80188, 90595, 38467, 28618, 13808, 28160, 85788, 77270, 64079, 46544, 35742, 65829, 95499, 72622, 30986, 62161, 40388, 68670, 64344, 30335, 19149, 97430, 41484]));
    }

    #[test]
    fn test2() {
        assert_eq!(1, run2(2, 12, 5, vec![1000, 2000]));
        assert_eq!(3, run2(3, 21, -11, vec![81234, 94124, 52141]));
        assert_eq!(449, run2(1000, 33, -60, vec![104, 78360, 32508, 56373, 76117, 50118, 1899, 59880, 44803, 76957, 49649, 40521, 47071, 89211, 9964, 57730, 46448, 10519, 10652, 16973, 12841, 90415, 33163, 60513, 11391, 87986, 64119, 70472, 76919, 44026, 17074, 83943, 62647, 57583, 48199, 86982, 44138, 53726, 35907, 48086, 37048, 78646, 15737, 66240, 19095, 68407, 69424, 99647, 84806, 38466, 97408, 54831, 38963, 77248, 48049, 34077, 41360, 52187, 54590, 70802, 13519, 48855, 82349, 72516, 74310, 7246, 47976, 5411, 52026, 16367, 77126, 63478, 77383, 26682, 35248, 9999, 23963, 2414, 94652, 40659, 79738, 23356, 93374, 21177, 39803, 55618, 51244, 85491, 96217, 51276, 71113, 86731, 53704, 54864, 95347, 94088, 18448, 90171, 48970, 1273, 23760, 95984, 44604, 76659, 81689, 43989, 75208, 72914, 86445, 91897, 10289, 65413, 95450, 87315, 50434, 30753, 78000, 37316, 56687, 61115, 94985, 2225, 77283, 67318, 63754, 83479, 81160, 12781, 55077, 33074, 37400, 97454, 70833, 53830, 46809, 25356, 84780, 18067, 95762, 39819, 83936, 47013, 1158, 75540, 6366, 81675, 26628, 14904, 26620, 54944, 53415, 13475, 44746, 8669, 1320, 39553, 11511, 96908, 87221, 20387, 41216, 59239, 33526, 60296, 17671, 36476, 6002, 86369, 55804, 18116, 32159, 19918, 90262, 80429, 18475, 63806, 68252, 18739, 14203, 5382, 34098, 73674, 51296, 26838, 66570, 67705, 6396, 92204, 21566, 46409, 26705, 92688, 91122, 31118, 30887, 73070, 54452, 67255, 11332, 88577, 61131, 81781, 96840, 21825, 66019, 50635, 71042, 69803, 84529, 79693, 59074, 5301, 49715, 81683, 8873, 44121, 32126, 5190, 28403, 37227, 79493, 25147, 89476, 84047, 54090, 62523, 32050, 36702, 69795, 15550, 96348, 1164, 56659, 23842, 62915, 62708, 85607, 48357, 13217, 87372, 41693, 85584, 98439, 6132, 96876, 36934, 62978, 99750, 58103, 31762, 43242, 5528, 75292, 27863, 1313, 11756, 59118, 4400, 83966, 13440, 56873, 27868, 67392, 93813, 523, 26, 5927, 76977, 92949, 98702, 49280, 97101, 74439, 71377, 97441, 44983, 59064, 31855, 64661, 19391, 64423, 27938, 3350, 27498, 71023, 97832, 11527, 54245, 91176, 51001, 81286, 51474, 11930, 19325, 97433, 67801, 58038, 77475, 50262, 9578, 56410, 25801, 51561, 61328, 61156, 41296, 96231, 10666, 57105, 51557, 70102, 28254, 5908, 26726, 22359, 33553, 45256, 20096, 76250, 57663, 28249, 15618, 15033, 46508, 75328, 85142, 92299, 97724, 46724, 12579, 96457, 17938, 36365, 74400, 82839, 55620, 23293, 41774, 60120, 47813, 81964, 79702, 4423, 20202, 58692, 14164, 84498, 20170, 51248, 67569, 36764, 32949, 80346, 9329, 81437, 69849, 94387, 41491, 67880, 76125, 89519, 30261, 63960, 45862, 52797, 63312, 16956, 91049, 24964, 26850, 46314, 95185, 18619, 82943, 95297, 75001, 60837, 95609, 52791, 85352, 4643, 27445, 21688, 2123, 99974, 11255, 61396, 60063, 81111, 39180, 70154, 80539, 25635, 63984, 4424, 67129, 43116, 50112, 6607, 9927, 76537, 98156, 64817, 65913, 3729, 44673, 79176, 2581, 58621, 24993, 62505, 36004, 16556, 1131, 54426, 91955, 66806, 9342, 16691, 16552, 91592, 99217, 90390, 781, 74778, 55948, 90113, 16844, 16302, 522, 17706, 4584, 19826, 17246, 28200, 34823, 79795, 907, 837, 51519, 38389, 23559, 76322, 25970, 20043, 73532, 17327, 76138, 15453, 25039, 46221, 86019, 38741, 29728, 32359, 62273, 74129, 93265, 85814, 16009, 12574, 34410, 14372, 65305, 47781, 85711, 36011, 28158, 59039, 86673, 23828, 30199, 34636, 42158, 55674, 55829, 35247, 50354, 64299, 43901, 25053, 38195, 48651, 37013, 91010, 81028, 71980, 6983, 42620, 87288, 43603, 99795, 12165, 13129, 37429, 60730, 29663, 69655, 52619, 59839, 95393, 33421, 67352, 83012, 98956, 58237, 56391, 72570, 45038, 83134, 82339, 83590, 1039, 84655, 99770, 81112, 54989, 47686, 82299, 67522, 74152, 60331, 71182, 10861, 64762, 56583, 83412, 32116, 36756, 27522, 72897, 61837, 24764, 38049, 52114, 98947, 74843, 53103, 40259, 48332, 31971, 86765, 37454, 91542, 72502, 61011, 26540, 94054, 70064, 70493, 10193, 58313, 51202, 79320, 28202, 44818, 96622, 47251, 69198, 64021, 26336, 93899, 63374, 95790, 26220, 74870, 87231, 52882, 62712, 87628, 53340, 74488, 77092, 79398, 74595, 7797, 70340, 56746, 78158, 8614, 7006, 10422, 33897, 47079, 78680, 67677, 58287, 64695, 95640, 65188, 58334, 25899, 91481, 65490, 90798, 2805, 81396, 71453, 4232, 74407, 22834, 6535, 4993, 92670, 33852, 35710, 38022, 13021, 86526, 87230, 90757, 6163, 30479, 25468, 98735, 27224, 41741, 47475, 32977, 3437, 94269, 29085, 22522, 22425, 36894, 58747, 19806, 42682, 11758, 16702, 32590, 583, 11764, 8581, 3763, 51652, 70053, 45170, 45448, 70894, 13140, 57259, 32577, 88684, 59772, 19032, 97221, 54705, 80126, 64582, 13127, 61591, 48105, 87411, 54903, 15571, 44104, 8397, 24438, 32624, 38656, 6974, 74570, 97227, 41794, 6020, 79855, 35643, 69462, 69333, 69433, 70500, 42761, 99963, 44589, 28309, 76660, 55893, 71468, 41874, 78814, 93390, 53535, 46327, 33136, 83871, 12883, 63345, 94288, 51612, 58530, 57662, 55351, 18689, 47469, 59241, 45422, 23630, 8373, 11076, 2057, 3760, 8026, 36435, 61079, 17096, 86321, 66646, 98422, 67976, 54845, 64156, 747, 88792, 89201, 3486, 90639, 77041, 35449, 14483, 94399, 91604, 84560, 13628, 13254, 47060, 82745, 91848, 36152, 86779, 81335, 51521, 36348, 95962, 66158, 46982, 22375, 66798, 35307, 90568, 20545, 79047, 84293, 58443, 62616, 65581, 24029, 14602, 21854, 92195, 52843, 91105, 18181, 96392, 43810, 33209, 16110, 82010, 62229, 93093, 38666, 65877, 90679, 55529, 22295, 32804, 85590, 97475, 59853, 36630, 28835, 83121, 23167, 94272, 11, 84027, 41066, 24380, 62896, 57615, 30812, 8510, 30060, 48965, 54834, 15667, 43216, 61555, 64901, 9913, 60737, 18344, 9941, 80263, 70020, 66465, 54352, 65192, 68069, 47118, 37422, 75123, 90177, 94817, 70601, 85362, 61248, 76797, 73939, 86624, 86234, 23791, 60332, 96620, 35996, 26078, 25066, 72893, 51547, 57101, 96678, 83085, 33599, 32204, 14890, 64189, 70137, 9663, 70226, 49418, 86224, 49854, 51376, 99275, 84610, 88138, 88993, 81865, 21482, 89874, 96118, 34538, 74323, 95855, 88687, 31060, 66072, 92993, 90861, 41166, 32059, 3791, 45992, 84052, 15314, 80493, 5111, 51526, 43178, 82272, 84262, 68099, 58073, 84871, 83020, 47061, 38242, 5725, 19624, 69539, 64402, 10046, 80220, 70891, 99246, 25188, 37490, 77067, 36646, 49273, 20538, 35833, 43636, 13841, 48611, 21841, 84579, 46350, 47332, 4416, 11377, 76446, 41884, 96130, 29299, 34609, 83446, 5223, 36456, 26949, 74975, 29776, 7146, 44879, 45112, 10566, 74318, 54553, 63333, 93877, 84684, 13173, 15107, 144, 69453, 90399, 6328, 53669, 88546, 53866, 65405, 43016, 66246, 82942, 14898, 9972, 49721, 78138, 58566, 34290, 61851, 79393, 22641, 60297, 34620, 38166, 26930, 92148, 94779, 74968, 57436, 92985, 70574, 47928, 18316, 90285, 11010, 45115, 87700, 97215, 85809, 50469, 84306, 39674, 56279, 96077, 76014, 180, 15251, 47146, 86795, 17594, 51232, 43958, 71749, 90413, 55329, 33855, 56606, 80324, 84816, 69970, 50925, 17099, 36064, 82328, 63715, 80188, 90595, 38467, 28618, 13808, 28160, 85788, 77270, 64079, 46544, 35742, 65829, 95499, 72622, 30986, 62161, 40388, 68670, 64344, 30335, 19149, 97430, 41484]));
        assert_eq!(775, run2(1000, 50, 50, vec![316, 76451, 26117, 54109, 74969, 46374, 1908, 59209, 40122, 75683, 46004, 36285, 42834, 87771, 7740, 56003, 42366, 8341, 8424, 13834, 9912, 89114, 27341, 60271, 9436, 86744, 63387, 69827, 75680, 39123, 13878, 82663, 62103, 55789, 44590, 85795, 39274, 50354, 30151, 44374, 32615, 76760, 12911, 65646, 15363, 67963, 68133, 98831, 83485, 34607, 96883, 51788, 35700, 76052, 44368, 28301, 36911, 49103, 51690, 70478, 10568, 45113, 80751, 71687, 73123, 6278, 44253, 4558, 48935, 13114, 75940, 62826, 76202, 21541, 29592, 7833, 18968, 2044, 93494, 36322, 77799, 18005, 92810, 16769, 35929, 52833, 47327, 83751, 95662, 47374, 70882, 85610, 50253, 52332, 93750, 93208, 14844, 88319, 45275, 1719, 18476, 95206, 39810, 75576, 80019, 39119, 74240, 72715, 85192, 91107, 8161, 64957, 94234, 86377, 46877, 25169, 76276, 32904, 54707, 60817, 93587, 1996, 76154, 66384, 63027, 82230, 79520, 9900, 52431, 26933, 32932, 97270, 70539, 50398, 42582, 20421, 83482, 14689, 94946, 36211, 82553, 42727, 1653, 74859, 5719, 79801, 21537, 11696, 21509, 52337, 49899, 10551, 39973, 6717, 1895, 35806, 9554, 96404, 85832, 16566, 36840, 58449, 27686, 59480, 14634, 31390, 5047, 85022, 53330, 14700, 26018, 16243, 88757, 78488, 14927, 63085, 67910, 15335, 10810, 4200, 28384, 72884, 47429, 21870, 66078, 66902, 5802, 91412, 16846, 42117, 21710, 91963, 90527, 25583, 25296, 72776, 51333, 66379, 9213, 87035, 60947, 80042, 96356, 16939, 65373, 47033, 70828, 68629, 83056, 77722, 58018, 4055, 46176, 79867, 6895, 39196, 25974, 3944, 23726, 32840, 77600, 20299, 87797, 82878, 50813, 61915, 25841, 31698, 68477, 12421, 95854, 1702, 54622, 18921, 62331, 62135, 84003, 44819, 10522, 86406, 36971, 83936, 97658, 5401, 96400, 32103, 62443, 98882, 56969, 25613, 38876, 4564, 74788, 22348, 1891, 9638, 58351, 2899, 82740, 10550, 54893, 22472, 66556, 92887, 523, 245, 4939, 75702, 92109, 97821, 45483, 96431, 73448, 71117, 97186, 40436, 57949, 25664, 64167, 15841, 64064, 22548, 2222, 22131, 70794, 97447, 9625, 51001, 90570, 47117, 79538, 47510, 9767, 15507, 97129, 66967, 56607, 76273, 46515, 7139, 54451, 21075, 48768, 60997, 60954, 36842, 95760, 8510, 55259, 48360, 69337, 23156, 4814, 21787, 17651, 27780, 41038, 16494, 75255, 55903, 23120, 12630, 11803, 42369, 74855, 83622, 91426, 97439, 42528, 9877, 95988, 14672, 31367, 73394, 80823, 53128, 17844, 37446, 59465, 43997, 80266, 77783, 3330, 16558, 57363, 10774, 83036, 16539, 47365, 66678, 32000, 26748, 78169, 6903, 79763, 68819, 93474, 36969, 67437, 74978, 87997, 24866, 63120, 41428, 49557, 62677, 13780, 90119, 19425, 21927, 41610, 93602, 15116, 80988, 93738, 74113, 60606, 94850, 49319, 83626, 3783, 22128, 16927, 1927, 99585, 9031, 61069, 59334, 79429, 35776, 69460, 79306, 20895, 63142, 3411, 66185, 38722, 46370, 5969, 7651, 75536, 97541, 64479, 65347, 2442, 39930, 77021, 2066, 57355, 19620, 61832, 30233, 13338, 1638, 51199, 91168, 66149, 6920, 13422, 13296, 90732, 98099, 88826, 922, 73827, 53771, 88230, 13568, 13061, 442, 14655, 3574, 16220, 14114, 22795, 29312, 77833, 1030, 948, 47615, 34523, 18373, 75306, 21222, 16424, 72861, 14316, 75174, 12227, 19812, 41552, 84639, 35333, 24337, 26109, 61314, 73033, 92592, 84465, 12948, 9787, 28854, 10881, 64886, 43944, 84167, 30520, 22594, 57935, 85434, 18658, 24673, 29222, 38324, 53149, 53467, 29532, 46655, 63701, 38998, 19841, 34227, 45006, 32277, 89897, 79318, 71453, 6193, 38503, 86305, 38889, 99302, 9771, 10131, 33251, 60283, 24280, 68386, 49152, 59178, 94012, 27664, 66517, 81505, 98095, 56986, 54332, 71743, 40447, 81975, 80570, 82256, 1434, 83214, 99161, 79444, 52373, 43498, 80503, 66592, 73082, 59806, 71028, 8562, 64382, 54513, 81984, 25967, 31745, 22185, 72310, 61274, 19258, 34153, 48941, 98023, 73935, 49853, 36212, 44767, 25702, 85693, 33351, 90673, 71580, 60644, 21477, 93203, 69228, 70123, 8134, 57031, 47188, 77179, 22931, 40205, 96153, 43276, 68107, 63216, 21437, 93025, 62820, 94951, 21418, 73944, 86170, 49712, 62216, 86573, 49879, 73547, 75880, 77587, 73735, 6305, 69519, 54880, 76393, 6715, 6243, 8307, 28123, 42867, 76851, 66890, 57024, 64258, 94866, 64653, 57128, 21106, 90665, 64981, 89548, 2138, 79674, 71163, 2858, 73437, 17795, 5859, 3803, 91649, 27907, 29803, 33992, 9937, 85315, 86125, 89450, 5462, 25083, 20604, 97836, 22074, 37232, 43486, 26757, 2249, 93212, 24197, 17736, 17725, 32008, 57378, 16146, 38566, 9669, 13435, 26340, 868, 9687, 6637, 2812, 48897, 69211, 40982, 41409, 70677, 10249, 55315, 26159, 87099, 59000, 15350, 96469, 51704, 77892, 64131, 10078, 61238, 44425, 86412, 52336, 12422, 39127, 6426, 19195, 26414, 35021, 6091, 73588, 96547, 37466, 5378, 77871, 29752, 68347, 68124, 68136, 70234, 38663, 99512, 39765, 23507, 75603, 53582, 71217, 37615, 76894, 92817, 49988, 41841, 27176, 82468, 9923, 62791, 93373, 48787, 57229, 55876, 52467, 15260, 43405, 58656, 41095, 18446, 6358, 8992, 1910, 2530, 6343, 31374, 60650, 14020, 84946, 66098, 97549, 67594, 52263, 63390, 889, 87528, 87674, 2295, 89288, 75719, 29740, 10889, 93481, 90965, 83066, 10717, 10536, 42753, 80763, 90995, 31025, 85761, 79562, 47734, 31287, 95156, 65562, 42638, 17702, 66101, 29610, 89166, 16681, 77001, 83012, 57167, 61957, 65045, 19117, 11253, 16970, 91318, 49668, 90291, 14704, 95887, 38980, 27659, 12965, 80351, 61307, 92590, 35315, 65245, 89362, 52665, 17570, 26502, 83951, 97304, 59197, 31520, 23936, 81931, 17796, 93228, 51, 82799, 36630, 19163, 62292, 55871, 25225, 6542, 24651, 45235, 52165, 12644, 38875, 61144, 64504, 7241, 60558, 14831, 7683, 78071, 69172, 65991, 51031, 64689, 67665, 42985, 33207, 74224, 88499, 93535, 70402, 83643, 60975, 75638, 72997, 85352, 84781, 18656, 60023, 96125, 30217, 21237, 20228, 71892, 48248, 55155, 96295, 81921, 27867, 26031, 11584, 63495, 69376, 7141, 69498, 45986, 84780, 46316, 47494, 98781, 83109, 86920, 87655, 80126, 16808, 88117, 95470, 28871, 73303, 94978, 87445, 25513, 65478, 92161, 89693, 36760, 25870, 2839, 41430, 82888, 12161, 78997, 3829, 48187, 38809, 80482, 82993, 67887, 56775, 83579, 81640, 42804, 34391, 4674, 15877, 68383, 63839, 7843, 77971, 70548, 98387, 20351, 33968, 75744, 31652, 45448, 16570, 30100, 38978, 10749, 44864, 16969, 83079, 41899, 43392, 3154, 9308, 75445, 38099, 95500, 24236, 29008, 81989, 3949, 31378, 22027, 73973, 24438, 6264, 40425, 40834, 8378, 73226, 51686, 62716, 92962, 83231, 10255, 11826, 360, 68174, 88865, 5613, 50068, 86981, 50452, 64899, 38670, 65848, 80927, 11628, 7779, 46304, 76302, 57289, 28541, 61281, 77386, 17792, 59628, 29212, 34181, 21935, 91271, 93523, 73964, 55428, 92138, 70314, 44163, 14714, 88802, 8636, 40913, 86699, 96468, 84418, 47000, 83021, 35887, 53996, 95251, 74928, 372, 11954, 43181, 85768, 14392, 47274, 39083, 71294, 88900, 52443, 27950, 54514, 78142, 83575, 69068, 47058, 14084, 30758, 80566, 63019, 77955, 89267, 34821, 23731, 10746, 22705, 84275, 76061, 63271, 42408, 30006, 65184, 94302, 71754, 25347, 61286, 36276, 68072, 63783, 25009, 15416, 97050, 36918]));
    }

    /*
    #[test]
    fn test3() {
        assert_eq!(1, run3(2, 12, 5, vec![1000, 2000]));
        assert_eq!(3, run3(3, 21, -11, vec![81234, 94124, 52141]));
        assert_eq!(449, run3(1000, 33, -60, vec![104, 78360, 32508, 56373, 76117, 50118, 1899, 59880, 44803, 76957, 49649, 40521, 47071, 89211, 9964, 57730, 46448, 10519, 10652, 16973, 12841, 90415, 33163, 60513, 11391, 87986, 64119, 70472, 76919, 44026, 17074, 83943, 62647, 57583, 48199, 86982, 44138, 53726, 35907, 48086, 37048, 78646, 15737, 66240, 19095, 68407, 69424, 99647, 84806, 38466, 97408, 54831, 38963, 77248, 48049, 34077, 41360, 52187, 54590, 70802, 13519, 48855, 82349, 72516, 74310, 7246, 47976, 5411, 52026, 16367, 77126, 63478, 77383, 26682, 35248, 9999, 23963, 2414, 94652, 40659, 79738, 23356, 93374, 21177, 39803, 55618, 51244, 85491, 96217, 51276, 71113, 86731, 53704, 54864, 95347, 94088, 18448, 90171, 48970, 1273, 23760, 95984, 44604, 76659, 81689, 43989, 75208, 72914, 86445, 91897, 10289, 65413, 95450, 87315, 50434, 30753, 78000, 37316, 56687, 61115, 94985, 2225, 77283, 67318, 63754, 83479, 81160, 12781, 55077, 33074, 37400, 97454, 70833, 53830, 46809, 25356, 84780, 18067, 95762, 39819, 83936, 47013, 1158, 75540, 6366, 81675, 26628, 14904, 26620, 54944, 53415, 13475, 44746, 8669, 1320, 39553, 11511, 96908, 87221, 20387, 41216, 59239, 33526, 60296, 17671, 36476, 6002, 86369, 55804, 18116, 32159, 19918, 90262, 80429, 18475, 63806, 68252, 18739, 14203, 5382, 34098, 73674, 51296, 26838, 66570, 67705, 6396, 92204, 21566, 46409, 26705, 92688, 91122, 31118, 30887, 73070, 54452, 67255, 11332, 88577, 61131, 81781, 96840, 21825, 66019, 50635, 71042, 69803, 84529, 79693, 59074, 5301, 49715, 81683, 8873, 44121, 32126, 5190, 28403, 37227, 79493, 25147, 89476, 84047, 54090, 62523, 32050, 36702, 69795, 15550, 96348, 1164, 56659, 23842, 62915, 62708, 85607, 48357, 13217, 87372, 41693, 85584, 98439, 6132, 96876, 36934, 62978, 99750, 58103, 31762, 43242, 5528, 75292, 27863, 1313, 11756, 59118, 4400, 83966, 13440, 56873, 27868, 67392, 93813, 523, 26, 5927, 76977, 92949, 98702, 49280, 97101, 74439, 71377, 97441, 44983, 59064, 31855, 64661, 19391, 64423, 27938, 3350, 27498, 71023, 97832, 11527, 54245, 91176, 51001, 81286, 51474, 11930, 19325, 97433, 67801, 58038, 77475, 50262, 9578, 56410, 25801, 51561, 61328, 61156, 41296, 96231, 10666, 57105, 51557, 70102, 28254, 5908, 26726, 22359, 33553, 45256, 20096, 76250, 57663, 28249, 15618, 15033, 46508, 75328, 85142, 92299, 97724, 46724, 12579, 96457, 17938, 36365, 74400, 82839, 55620, 23293, 41774, 60120, 47813, 81964, 79702, 4423, 20202, 58692, 14164, 84498, 20170, 51248, 67569, 36764, 32949, 80346, 9329, 81437, 69849, 94387, 41491, 67880, 76125, 89519, 30261, 63960, 45862, 52797, 63312, 16956, 91049, 24964, 26850, 46314, 95185, 18619, 82943, 95297, 75001, 60837, 95609, 52791, 85352, 4643, 27445, 21688, 2123, 99974, 11255, 61396, 60063, 81111, 39180, 70154, 80539, 25635, 63984, 4424, 67129, 43116, 50112, 6607, 9927, 76537, 98156, 64817, 65913, 3729, 44673, 79176, 2581, 58621, 24993, 62505, 36004, 16556, 1131, 54426, 91955, 66806, 9342, 16691, 16552, 91592, 99217, 90390, 781, 74778, 55948, 90113, 16844, 16302, 522, 17706, 4584, 19826, 17246, 28200, 34823, 79795, 907, 837, 51519, 38389, 23559, 76322, 25970, 20043, 73532, 17327, 76138, 15453, 25039, 46221, 86019, 38741, 29728, 32359, 62273, 74129, 93265, 85814, 16009, 12574, 34410, 14372, 65305, 47781, 85711, 36011, 28158, 59039, 86673, 23828, 30199, 34636, 42158, 55674, 55829, 35247, 50354, 64299, 43901, 25053, 38195, 48651, 37013, 91010, 81028, 71980, 6983, 42620, 87288, 43603, 99795, 12165, 13129, 37429, 60730, 29663, 69655, 52619, 59839, 95393, 33421, 67352, 83012, 98956, 58237, 56391, 72570, 45038, 83134, 82339, 83590, 1039, 84655, 99770, 81112, 54989, 47686, 82299, 67522, 74152, 60331, 71182, 10861, 64762, 56583, 83412, 32116, 36756, 27522, 72897, 61837, 24764, 38049, 52114, 98947, 74843, 53103, 40259, 48332, 31971, 86765, 37454, 91542, 72502, 61011, 26540, 94054, 70064, 70493, 10193, 58313, 51202, 79320, 28202, 44818, 96622, 47251, 69198, 64021, 26336, 93899, 63374, 95790, 26220, 74870, 87231, 52882, 62712, 87628, 53340, 74488, 77092, 79398, 74595, 7797, 70340, 56746, 78158, 8614, 7006, 10422, 33897, 47079, 78680, 67677, 58287, 64695, 95640, 65188, 58334, 25899, 91481, 65490, 90798, 2805, 81396, 71453, 4232, 74407, 22834, 6535, 4993, 92670, 33852, 35710, 38022, 13021, 86526, 87230, 90757, 6163, 30479, 25468, 98735, 27224, 41741, 47475, 32977, 3437, 94269, 29085, 22522, 22425, 36894, 58747, 19806, 42682, 11758, 16702, 32590, 583, 11764, 8581, 3763, 51652, 70053, 45170, 45448, 70894, 13140, 57259, 32577, 88684, 59772, 19032, 97221, 54705, 80126, 64582, 13127, 61591, 48105, 87411, 54903, 15571, 44104, 8397, 24438, 32624, 38656, 6974, 74570, 97227, 41794, 6020, 79855, 35643, 69462, 69333, 69433, 70500, 42761, 99963, 44589, 28309, 76660, 55893, 71468, 41874, 78814, 93390, 53535, 46327, 33136, 83871, 12883, 63345, 94288, 51612, 58530, 57662, 55351, 18689, 47469, 59241, 45422, 23630, 8373, 11076, 2057, 3760, 8026, 36435, 61079, 17096, 86321, 66646, 98422, 67976, 54845, 64156, 747, 88792, 89201, 3486, 90639, 77041, 35449, 14483, 94399, 91604, 84560, 13628, 13254, 47060, 82745, 91848, 36152, 86779, 81335, 51521, 36348, 95962, 66158, 46982, 22375, 66798, 35307, 90568, 20545, 79047, 84293, 58443, 62616, 65581, 24029, 14602, 21854, 92195, 52843, 91105, 18181, 96392, 43810, 33209, 16110, 82010, 62229, 93093, 38666, 65877, 90679, 55529, 22295, 32804, 85590, 97475, 59853, 36630, 28835, 83121, 23167, 94272, 11, 84027, 41066, 24380, 62896, 57615, 30812, 8510, 30060, 48965, 54834, 15667, 43216, 61555, 64901, 9913, 60737, 18344, 9941, 80263, 70020, 66465, 54352, 65192, 68069, 47118, 37422, 75123, 90177, 94817, 70601, 85362, 61248, 76797, 73939, 86624, 86234, 23791, 60332, 96620, 35996, 26078, 25066, 72893, 51547, 57101, 96678, 83085, 33599, 32204, 14890, 64189, 70137, 9663, 70226, 49418, 86224, 49854, 51376, 99275, 84610, 88138, 88993, 81865, 21482, 89874, 96118, 34538, 74323, 95855, 88687, 31060, 66072, 92993, 90861, 41166, 32059, 3791, 45992, 84052, 15314, 80493, 5111, 51526, 43178, 82272, 84262, 68099, 58073, 84871, 83020, 47061, 38242, 5725, 19624, 69539, 64402, 10046, 80220, 70891, 99246, 25188, 37490, 77067, 36646, 49273, 20538, 35833, 43636, 13841, 48611, 21841, 84579, 46350, 47332, 4416, 11377, 76446, 41884, 96130, 29299, 34609, 83446, 5223, 36456, 26949, 74975, 29776, 7146, 44879, 45112, 10566, 74318, 54553, 63333, 93877, 84684, 13173, 15107, 144, 69453, 90399, 6328, 53669, 88546, 53866, 65405, 43016, 66246, 82942, 14898, 9972, 49721, 78138, 58566, 34290, 61851, 79393, 22641, 60297, 34620, 38166, 26930, 92148, 94779, 74968, 57436, 92985, 70574, 47928, 18316, 90285, 11010, 45115, 87700, 97215, 85809, 50469, 84306, 39674, 56279, 96077, 76014, 180, 15251, 47146, 86795, 17594, 51232, 43958, 71749, 90413, 55329, 33855, 56606, 80324, 84816, 69970, 50925, 17099, 36064, 82328, 63715, 80188, 90595, 38467, 28618, 13808, 28160, 85788, 77270, 64079, 46544, 35742, 65829, 95499, 72622, 30986, 62161, 40388, 68670, 64344, 30335, 19149, 97430, 41484]));
        assert_eq!(775, run3(1000, 50, 50, vec![316, 76451, 26117, 54109, 74969, 46374, 1908, 59209, 40122, 75683, 46004, 36285, 42834, 87771, 7740, 56003, 42366, 8341, 8424, 13834, 9912, 89114, 27341, 60271, 9436, 86744, 63387, 69827, 75680, 39123, 13878, 82663, 62103, 55789, 44590, 85795, 39274, 50354, 30151, 44374, 32615, 76760, 12911, 65646, 15363, 67963, 68133, 98831, 83485, 34607, 96883, 51788, 35700, 76052, 44368, 28301, 36911, 49103, 51690, 70478, 10568, 45113, 80751, 71687, 73123, 6278, 44253, 4558, 48935, 13114, 75940, 62826, 76202, 21541, 29592, 7833, 18968, 2044, 93494, 36322, 77799, 18005, 92810, 16769, 35929, 52833, 47327, 83751, 95662, 47374, 70882, 85610, 50253, 52332, 93750, 93208, 14844, 88319, 45275, 1719, 18476, 95206, 39810, 75576, 80019, 39119, 74240, 72715, 85192, 91107, 8161, 64957, 94234, 86377, 46877, 25169, 76276, 32904, 54707, 60817, 93587, 1996, 76154, 66384, 63027, 82230, 79520, 9900, 52431, 26933, 32932, 97270, 70539, 50398, 42582, 20421, 83482, 14689, 94946, 36211, 82553, 42727, 1653, 74859, 5719, 79801, 21537, 11696, 21509, 52337, 49899, 10551, 39973, 6717, 1895, 35806, 9554, 96404, 85832, 16566, 36840, 58449, 27686, 59480, 14634, 31390, 5047, 85022, 53330, 14700, 26018, 16243, 88757, 78488, 14927, 63085, 67910, 15335, 10810, 4200, 28384, 72884, 47429, 21870, 66078, 66902, 5802, 91412, 16846, 42117, 21710, 91963, 90527, 25583, 25296, 72776, 51333, 66379, 9213, 87035, 60947, 80042, 96356, 16939, 65373, 47033, 70828, 68629, 83056, 77722, 58018, 4055, 46176, 79867, 6895, 39196, 25974, 3944, 23726, 32840, 77600, 20299, 87797, 82878, 50813, 61915, 25841, 31698, 68477, 12421, 95854, 1702, 54622, 18921, 62331, 62135, 84003, 44819, 10522, 86406, 36971, 83936, 97658, 5401, 96400, 32103, 62443, 98882, 56969, 25613, 38876, 4564, 74788, 22348, 1891, 9638, 58351, 2899, 82740, 10550, 54893, 22472, 66556, 92887, 523, 245, 4939, 75702, 92109, 97821, 45483, 96431, 73448, 71117, 97186, 40436, 57949, 25664, 64167, 15841, 64064, 22548, 2222, 22131, 70794, 97447, 9625, 51001, 90570, 47117, 79538, 47510, 9767, 15507, 97129, 66967, 56607, 76273, 46515, 7139, 54451, 21075, 48768, 60997, 60954, 36842, 95760, 8510, 55259, 48360, 69337, 23156, 4814, 21787, 17651, 27780, 41038, 16494, 75255, 55903, 23120, 12630, 11803, 42369, 74855, 83622, 91426, 97439, 42528, 9877, 95988, 14672, 31367, 73394, 80823, 53128, 17844, 37446, 59465, 43997, 80266, 77783, 3330, 16558, 57363, 10774, 83036, 16539, 47365, 66678, 32000, 26748, 78169, 6903, 79763, 68819, 93474, 36969, 67437, 74978, 87997, 24866, 63120, 41428, 49557, 62677, 13780, 90119, 19425, 21927, 41610, 93602, 15116, 80988, 93738, 74113, 60606, 94850, 49319, 83626, 3783, 22128, 16927, 1927, 99585, 9031, 61069, 59334, 79429, 35776, 69460, 79306, 20895, 63142, 3411, 66185, 38722, 46370, 5969, 7651, 75536, 97541, 64479, 65347, 2442, 39930, 77021, 2066, 57355, 19620, 61832, 30233, 13338, 1638, 51199, 91168, 66149, 6920, 13422, 13296, 90732, 98099, 88826, 922, 73827, 53771, 88230, 13568, 13061, 442, 14655, 3574, 16220, 14114, 22795, 29312, 77833, 1030, 948, 47615, 34523, 18373, 75306, 21222, 16424, 72861, 14316, 75174, 12227, 19812, 41552, 84639, 35333, 24337, 26109, 61314, 73033, 92592, 84465, 12948, 9787, 28854, 10881, 64886, 43944, 84167, 30520, 22594, 57935, 85434, 18658, 24673, 29222, 38324, 53149, 53467, 29532, 46655, 63701, 38998, 19841, 34227, 45006, 32277, 89897, 79318, 71453, 6193, 38503, 86305, 38889, 99302, 9771, 10131, 33251, 60283, 24280, 68386, 49152, 59178, 94012, 27664, 66517, 81505, 98095, 56986, 54332, 71743, 40447, 81975, 80570, 82256, 1434, 83214, 99161, 79444, 52373, 43498, 80503, 66592, 73082, 59806, 71028, 8562, 64382, 54513, 81984, 25967, 31745, 22185, 72310, 61274, 19258, 34153, 48941, 98023, 73935, 49853, 36212, 44767, 25702, 85693, 33351, 90673, 71580, 60644, 21477, 93203, 69228, 70123, 8134, 57031, 47188, 77179, 22931, 40205, 96153, 43276, 68107, 63216, 21437, 93025, 62820, 94951, 21418, 73944, 86170, 49712, 62216, 86573, 49879, 73547, 75880, 77587, 73735, 6305, 69519, 54880, 76393, 6715, 6243, 8307, 28123, 42867, 76851, 66890, 57024, 64258, 94866, 64653, 57128, 21106, 90665, 64981, 89548, 2138, 79674, 71163, 2858, 73437, 17795, 5859, 3803, 91649, 27907, 29803, 33992, 9937, 85315, 86125, 89450, 5462, 25083, 20604, 97836, 22074, 37232, 43486, 26757, 2249, 93212, 24197, 17736, 17725, 32008, 57378, 16146, 38566, 9669, 13435, 26340, 868, 9687, 6637, 2812, 48897, 69211, 40982, 41409, 70677, 10249, 55315, 26159, 87099, 59000, 15350, 96469, 51704, 77892, 64131, 10078, 61238, 44425, 86412, 52336, 12422, 39127, 6426, 19195, 26414, 35021, 6091, 73588, 96547, 37466, 5378, 77871, 29752, 68347, 68124, 68136, 70234, 38663, 99512, 39765, 23507, 75603, 53582, 71217, 37615, 76894, 92817, 49988, 41841, 27176, 82468, 9923, 62791, 93373, 48787, 57229, 55876, 52467, 15260, 43405, 58656, 41095, 18446, 6358, 8992, 1910, 2530, 6343, 31374, 60650, 14020, 84946, 66098, 97549, 67594, 52263, 63390, 889, 87528, 87674, 2295, 89288, 75719, 29740, 10889, 93481, 90965, 83066, 10717, 10536, 42753, 80763, 90995, 31025, 85761, 79562, 47734, 31287, 95156, 65562, 42638, 17702, 66101, 29610, 89166, 16681, 77001, 83012, 57167, 61957, 65045, 19117, 11253, 16970, 91318, 49668, 90291, 14704, 95887, 38980, 27659, 12965, 80351, 61307, 92590, 35315, 65245, 89362, 52665, 17570, 26502, 83951, 97304, 59197, 31520, 23936, 81931, 17796, 93228, 51, 82799, 36630, 19163, 62292, 55871, 25225, 6542, 24651, 45235, 52165, 12644, 38875, 61144, 64504, 7241, 60558, 14831, 7683, 78071, 69172, 65991, 51031, 64689, 67665, 42985, 33207, 74224, 88499, 93535, 70402, 83643, 60975, 75638, 72997, 85352, 84781, 18656, 60023, 96125, 30217, 21237, 20228, 71892, 48248, 55155, 96295, 81921, 27867, 26031, 11584, 63495, 69376, 7141, 69498, 45986, 84780, 46316, 47494, 98781, 83109, 86920, 87655, 80126, 16808, 88117, 95470, 28871, 73303, 94978, 87445, 25513, 65478, 92161, 89693, 36760, 25870, 2839, 41430, 82888, 12161, 78997, 3829, 48187, 38809, 80482, 82993, 67887, 56775, 83579, 81640, 42804, 34391, 4674, 15877, 68383, 63839, 7843, 77971, 70548, 98387, 20351, 33968, 75744, 31652, 45448, 16570, 30100, 38978, 10749, 44864, 16969, 83079, 41899, 43392, 3154, 9308, 75445, 38099, 95500, 24236, 29008, 81989, 3949, 31378, 22027, 73973, 24438, 6264, 40425, 40834, 8378, 73226, 51686, 62716, 92962, 83231, 10255, 11826, 360, 68174, 88865, 5613, 50068, 86981, 50452, 64899, 38670, 65848, 80927, 11628, 7779, 46304, 76302, 57289, 28541, 61281, 77386, 17792, 59628, 29212, 34181, 21935, 91271, 93523, 73964, 55428, 92138, 70314, 44163, 14714, 88802, 8636, 40913, 86699, 96468, 84418, 47000, 83021, 35887, 53996, 95251, 74928, 372, 11954, 43181, 85768, 14392, 47274, 39083, 71294, 88900, 52443, 27950, 54514, 78142, 83575, 69068, 47058, 14084, 30758, 80566, 63019, 77955, 89267, 34821, 23731, 10746, 22705, 84275, 76061, 63271, 42408, 30006, 65184, 94302, 71754, 25347, 61286, 36276, 68072, 63783, 25009, 15416, 97050, 36918]));
    }
    */
}
