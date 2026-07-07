
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        if nums1.len() >= nums2.len() 
        {Self::find_median_rec(nums1,nums2)}
        else {Self::find_median_rec(nums2,nums1)}

        

    }


    pub fn find_median_rec(bigger:Vec<i32>, smaller: Vec<i32>) -> f64{
        let slen = smaller.len();
        let blen = bigger.len();
        let allsize = slen + blen;
        let mut smed = slen /2;
        let mut sleft = slen *0;
        let mut sright = slen -1;
        let bmedcalc = |smed|{allsize /2 - smed -1};
        let mut bmed = bmedcalc(smed);
        let mut searching = true;


        if slen == 0 {
            return (bigger[bmed] + bigger[bmed + (blen ^ 1) ]) as f64 /2.0
        }
        while {searching}{
            smed = (sleft + sright) /2;
            bmed = bmedcalc(smed);
            if !searching && smaller[smed] > bigger[bmed] {
                sright = smed;
            }else if !searching && smaller[smed]< bigger[bmed] {
                sleft = smed;
            }else{
                searching = false;
            }
            if sleft == sright{
                searching = false;
            } 
        }
        if (allsize &1 as usize ) == 1 {
            smaller[smed] as f64
        }else {
            (smaller[smed] + bigger[bmed]) as f64 / 2.0
        }
    }
}
