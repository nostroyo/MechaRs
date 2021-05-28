use crate::mecha::Mecha;
use crate::backend_mecha_function::BackEndMechaFunction;
use web3::types::U256;
use std::cmp;


const CACHE_SIZE: usize = 5;

pub struct CachedMechaCollection<F: BackEndMechaFunction> {
    mecha_list: Vec<Mecha>,
    index: usize,
    total_length: U256,
    backend_func: F,
}

impl<F: BackEndMechaFunction> Iterator for CachedMechaCollection<F> {
    type Item = Mecha;

    fn next(&mut self) -> Option<Self::Item> {
        if U256::from(self.index) > self.total_length {
            None
        } else {
            if self.index > self.mecha_list.len() {

                println!("{}", self.mecha_list.len());
                self.load_mecha_from_backend(self.index)

            }

            let mecha_opt = self.mecha_list.get(self.index);
            self.index += 1;
            match mecha_opt {
                None => {None}
                Some(mecha_instance) => {Some(mecha_instance.clone())}
            }

        }
    }
}

impl<F: BackEndMechaFunction> CachedMechaCollection<F> {

    pub fn new(backend_function: F) -> Self {

        CachedMechaCollection {
            mecha_list: vec![],
            index: 0,
            total_length: Default::default(),
            backend_func: backend_function
        }


    }

    pub fn load_mecha_from_backend(&mut self, offset: usize) {

        let total_length = self.backend_func.get_total_mecha_owned();
        self.total_length = total_length;

        for i in offset..cmp::min(offset + CACHE_SIZE, total_length.as_usize()) {
            self.mecha_list.push(
                Mecha::NewFromBlockchain(
                    self.backend_func.get_mecha_characteristics_by_id(U256::from(i))
                ).unwrap()
            );
        }


    }
}

#[cfg(test)]
mod tests {
    use crate::mecha_collection::CachedMechaCollection;
    use crate::mock_backend_function::MockBackend;
    use crate::backend_mecha_function::BackEndMechaFunction;


    pub fn setup_mock() -> CachedMechaCollection<MockBackend> {

        let mut collection = CachedMechaCollection::new(MockBackend::new());

        for i in 0..9 {
            collection.backend_func.generate_new_mecha(format!("{}", i));
        }
        collection
    }

    #[test]
    pub fn test_mock_cache() {

        let mut collection = setup_mock();

        collection.load_mecha_from_backend(0);
        assert_eq!(collection.mecha_list.len(), 5);

        collection.load_mecha_from_backend(5);
        assert_eq!(collection.mecha_list.len(), 9);

         for mecha in collection {
             println!("{}", mecha.name);

         }


    }
}
