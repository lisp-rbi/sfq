use crate::{Fdb,Get};


impl Fdb {

    pub fn colaps(&mut self) -> &mut Self {

/* Debug
        println!("Comp   {:?}", String::from_utf8(self.head.clone()).unwrap());
        println!("Comp   {:?}", String::from_utf8(self.seq.clone()).unwrap());
        println!("Comp   {:?}", String::from_utf8(self.qual.clone()).unwrap());
*/

        self.sort("h");

        let tsv : Vec<u8> = if self.paired == false {
            self.get_tsv("s+q+h")
        }else{
            let b = self.get_tsv("s+q");
            let h = self.get_head();

            let body : Vec<_> = b.split(|i| *i == 10u8).collect();
            let head : Vec<_> = h.split(|i| *i == 10u8).collect();
            let mut x = 0;
            let mut tmp = vec![0u8; self.head.len()+self.seq.len()+self.qual.len()+100];
            for i in (1..head.len()).step_by(2){
                let (mut m, mut n) = (0,0);
                for j in 0..body[i-1].len(){
                    tmp[x] = body[i-1][j];x+=1;
                    m+=1;
                    if body[i-1][j] == 9u8 {break;}
                }
                for j in 0..body[i].len(){
                    tmp[x] = body[i][j];x+=1;
                    n+=1;
                    if body[i][j] == 9u8 {break;}
                }

                for j in m..body[i-1].len(){
                    tmp[x] = body[i-1][j];x+=1;
                    if body[i-1][j] == 9u8 {break;}
                }
                tmp[x] = 9u8;x+=1;
                for j in n..body[i].len(){
                    tmp[x] = body[i][j];x+=1;
                    if body[i][j] == 9u8 {break;}
                }
                tmp[x] = 9u8;x+=1;
                for j in 0..head[i-1].len(){
                    tmp[x] = head[i-1][j];x+=1;
                }
                tmp[x] = 9u8;x+=1;
                for j in 0..head[i].len(){
                    tmp[x] = head[i][j];x+=1;
                }
                tmp[x] = 10u8;x+=1;
            }

            tmp.resize(x-1, 0x00);
            tmp
        };

        let mut sorted : Vec<_> = tsv.split(|i| *i == 10u8).collect();
        sorted.sort();
        if self.paired==true {
            if  self.filter_fwd_and_rev(&mut sorted) == false {
                panic!("Panic!!");
            };
        }else{
            if self.filter_fwd(&mut sorted) == false {
                panic!("Panic!!")
            };
        }

        self
    }

    fn filter_fwd(&mut self, sorted: &Vec<&[u8]>) -> bool {



        let (fs, fq, fh) = (0,1,2);
        let (mut h, mut s, mut q) = (0,0,0);
        let mut fc = 1;
        let mut a : Vec<_> = sorted[0].split(|i| *i == 9u8).collect();
        let mut qavg = vec![0f64; a[fq].len()];


        for i in 1..sorted.len(){
            let b : Vec<_> = sorted[i].split(|i| *i == 9u8).collect();
            self.qavrg(&(a[fq].to_vec()), &mut qavg, fc);


            if self.compare_vslice(a[fs], b[fs]) == false {
                fc=1;

                for i in a[fh].iter(){self.head[h] = *i;  h+=1;}
                self.head[h] = 10u8; h+=1;


                for i in a[fs].iter(){ self.seq[s] = *i; s+=1;}
                self.seq[s] = 10u8; s+=1;

                for i in qavg.iter(){self.qual[q] = *i as u8; q+=1;}
                self.qual[q] = 10u8; q+=1;

                qavg = vec![0f64;if a[fq].len()> b[fq].len() {a[fq].len()}else{b[fq].len()}];

            }else{
                fc+=1;
            }
            a = b;
        }

        let a : Vec<_> = sorted[sorted.len()-1].split(|i| *i == 9u8).collect();
        self.qavrg(&(a[fq].to_vec()), &mut qavg, fc);

        for i in a[fh].iter(){self.head[h] = *i; h+=1;}
        self.head[h] = 10u8; h+=1;

        for i in a[fs].iter(){ self.seq[s] = *i; s+=1;}
        self.seq[s] = 10u8; s+=1;

        for i in qavg.iter(){self.qual[q] = *i as u8; q+=1;}
        self.qual[q] = 10u8; q+=1;

        self.head.resize(h-1,0x00);
        self.seq.resize(s-1,0x00);
        self.qual.resize(q-1,0x00);

        true
    }

    fn filter_fwd_and_rev (&mut self, sorted: &Vec<&[u8]>) -> bool {


        let (fs, rs, fq, rq, fh, rh) =(0,1,2,3,4,5);
        let (mut h, mut s, mut q, mut c) = (0,0,0,0);
        let (mut fc, mut rc) = (1,1);
        let mut a : Vec<_> = sorted[0].split(|i| *i == 9u8).collect();
        let mut qfavg = vec![0f64; a[fq].len()];
        let mut qravg = vec![0f64; a[rq].len()];
        self.cpcnt = vec![0;self.head.len()];

        for i in 1..sorted.len(){
            let b : Vec<_> = sorted[i].split(|i| *i == 9u8).collect();

            self.qavrg(&(a[fq].to_vec()), &mut qfavg, fc);
            self.qavrg(&(a[rq].to_vec()), &mut qravg, rc);

            if self.compare_vslice(a[fs], b[fs]) == false ||
                self.compare_vslice(a[rs], b[rs]) == false {
                self.cpcnt[c] =rc+1;c+=1;
                rc=1;

                for i in a[rh].iter(){self.head[h] = *i; h+=1;}
                self.head[h] = 10u8; h+=1;

                for i in a[rs].iter(){self.seq[s] = *i; s+=1;}
                self.seq[s] = 10u8; s+=1;

                for i in qravg.iter(){self.qual[q] = *i as u8; q+=1;}
                self.qual[q] = 10u8; q+=1;

                qravg = vec![0f64;if a[rq].len()> b[rq].len() {a[rq].len()}else{b[rq].len()}];

            }else{
                rc+=1;
            }

            if self.compare_vslice(a[fs], b[fs]) == false {
                self.cpcnt[c] =fc+1;c+=1;
                fc=1;

                for i in a[fh].iter(){self.head[h] = *i; h+=1;}
                self.head[h] = 10u8; h+=1;

                for i in a[fs].iter(){ self.seq[s] = *i; s+=1;}
                self.seq[s] = 10u8; s+=1;

                for i in qfavg.iter(){self.qual[q] = *i as u8; q+=1;}
                self.qual[q] = 10u8; q+=1;
                qfavg = vec![0f64; if a[fq].len()> b[fq].len() {a[fq].len()}else{b[fq].len()}];
            }else{
                fc+=1;
            }

            a=b;
        }

        let a : Vec<_> = sorted[sorted.len()-1].split(|i| *i == 9u8).collect();
        self.qavrg(&(a[fq].to_vec()), &mut qfavg, fc);
        self.qavrg(&(a[rq].to_vec()), &mut qravg, rc);
        self.cpcnt[c] =rc+1;c+=1;self.cpcnt[c] =fc+1;c+=1;
        self.cpcnt.resize(c,0x00);




        for i in a[rh].iter(){self.head[h] = *i; h+=1;}
        self.head[h] = 10u8; h+=1;

        for i in a[fh].iter(){self.head[h] = *i; h+=1;}
        self.head[h] = 10u8;

        for i in a[rs].iter(){self.seq[s] = *i; s+=1;}
        self.seq[s] = 10u8; s+=1;

        for i in a[fs].iter(){ self.seq[s] = *i; s+=1;}
        self.seq[s] = 10u8;

        for i in qravg.iter(){self.qual[q] = *i as u8; q+=1;}
        self.qual[q] = 10u8; q+=1;

        for i in qfavg.iter(){self.qual[q] = *i as u8; q+=1;}
        self.qual[q] = 10u8;

        self.head.resize(h,0x00);
        self.seq.resize(s,0x00);
        self.qual.resize(q,0x00);

/*  Debug
        println!("Deflated   {:?}", String::from_utf8(self.head.clone()).unwrap());
        println!("Deflated   {:?}", String::from_utf8(self.seq.clone()).unwrap());
        println!("Deflated   {:?}", String::from_utf8(self.qual.clone()).unwrap());
*/
        true

    }


    pub fn expand(&mut self) -> &mut Self {

        let data = self.get_tsv("h+s+q");

        let mut size = 0;
        let mut max  = 0;

        for i in self.cpcnt.iter() {
            if *i > max {max= *i};
        }
        size = max * self.seq.len();

        let mut h = vec![0u8;size];
        let mut s = vec![0u8;size];
        let mut q = vec![0u8;size];

        let mut buff_h : Vec<Vec<u8>> = Vec::new();


        let (mut bh, mut bs, mut bq, mut eh, mut es, mut eq) = (0,0,0,0,0,0);

        let tsv : Vec<_> = data.split(|i| *i == 10u8).collect();

        for i in 0..tsv.len() {

            //eprintln!(">>>>{:?}", String::from_utf8(tsv[i].to_vec()).unwrap());

            let a : Vec<_> = tsv[i].split(|i| *i == 9u8).collect();

            if a[0][a[0].len()-1] == 'F' as u8 {

                while buff_h.len() >0 {
                    let hf =  buff_h.pop().unwrap();
                    eh = bh + hf.len();
                    es = bs + a[1].len();
                    eq = bq + a[2].len();
                    (&mut h[bh..eh]).copy_from_slice(&hf);
                    (&mut s[bs..es]).copy_from_slice(&a[1]);
                    (&mut q[bq..eq]).copy_from_slice(&a[2]);
                    h[eh] = 10u8;
                    s[es] = 10u8;
                    q[eq] = 10u8;
                    bh = eh+1;
                    bs = es+1;
                    bq = eq+1;
                }
                //eprintln!("buff_h {:?}\nh {:?}\ns {:?}\nq {:?}\n",buff_h, h,s,q);


            }else{

                let hd = a[0].to_vec();
                //eprintln!("cpcnt  {:?}",self.cpcnt[i]);
                for j in 1..self.cpcnt[i] {

                    let mut  hr = hd.clone();
                    hr.pop();
                    let dig = j.to_string().as_bytes().to_vec();
                    hr.extend(dig);

                    //println!("HEREXC ------ {:?}", hr_in);
                    let mut hf = hr.clone();

                    hf.push('F' as u8);
                    hr.push('R' as u8);
                    buff_h.push(hf);



                    eh = bh + hr.len();
                    es = bs + a[1].len();
                    eq = bq + a[2].len();
                    (&mut h[bh..eh]).copy_from_slice(&hr);
                    (&mut s[bs..es]).copy_from_slice(&a[1]);
                    (&mut q[bq..eq]).copy_from_slice(&a[2]);
                    h[eh] = 10u8;
                    s[es] = 10u8;
                    q[eq] = 10u8;
                    bh = eh+1;
                    bs = es+1;
                    bq = eq+1;

                }

            };

        }

        h.resize(eh,0x00);
        s.resize(es,0x00);
        q.resize(eq,0x00);
        self.head = h;
        self.seq = s;
        self.qual = q;

        self.sort("h");


        self

    }


}
