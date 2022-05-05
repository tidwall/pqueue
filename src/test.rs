#[cfg(test)]
use super::*;

#[test]
fn queue() {
    let mut q = Queue::new();
    let unordered = [
        130, 354, 307, 82, 773, 18, 944, 849, 293, 403, 532, 666, 990, 187, 902, 772, 427, 450,
        885, 476, 92, 221, 822, 744, 578, 942, 218, 229, 392, 254, 436, 687, 277, 364, 167, 316,
        630, 665, 345, 600, 186, 862, 129, 309, 789, 134, 799, 347, 558, 805, 549, 112, 494, 679,
        146, 778, 577, 610, 482, 289, 511, 766, 439, 721, 507, 682, 752, 330, 499, 628, 787, 429,
        888, 272, 109, 846, 573, 416, 105, 915, 626, 4, 980, 123, 985, 226, 182, 661, 95, 28, 546,
        361, 833, 243, 906, 957, 32, 405, 255, 457,
    ];
    let mut ordered = Vec::new();
    let mut count = 0;
    for item in unordered {
        q.push(item);
        count += 1;
        assert_eq!(q.len(), count);
        ordered.push(item);
        ordered.sort();
        if let Some(item) = q.peek() {
            assert_eq!(item, &ordered[0]);
        }
    }
    count = 0;
    while let Some(item) = q.pop() {
        count += 1;
        assert_eq!(q.len(), unordered.len()-count);
        assert_eq!(item, ordered[count-1]);
    }
}
