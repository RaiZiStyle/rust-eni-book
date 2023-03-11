#![allow(unused)]

pub mod mod_animal {

    /// Création de l'interface avec 4 fonction
    /// * creer(String) : Renvoi l'instance Crée d'un Animal
    /// * emettre_son(&self) : Renvoi le son de l'Animal
    /// * obtenir_nom(&self) : Renvoi le nom de l'Animal
    /// * afficher(&self) :  affiche le nom et le son de l'Animal
    pub trait Animal {
        fn creer(nom: String) -> Self;

        fn emettre_son(&self) -> String;
        fn obtenir_nom(&self) -> String;

        fn afficher(&self) {
            println!("{} : {}", self.obtenir_nom(), self.emettre_son());
        }
    }

    pub fn afficher_trait<T: Animal>(animal: &T) {
        animal.afficher();
    }

    

    // pub fn getbiggestname(animal1: &impl Animal, animal2: &impl Animal) -> String
    // pub fn getbiggestname<T: Animal>(animal1: &T, animal2: &T) -> String # Sucre syntaxique
    pub fn getbiggestname(animal1: &impl Animal, animal2: &impl Animal) -> String {
        if animal1.obtenir_nom().len() > animal2.obtenir_nom().len() {
            return animal1.obtenir_nom();
        } else if animal1.obtenir_nom().len() < animal2.obtenir_nom().len() {
            return animal2.obtenir_nom();
        }
        "Même longueur".to_string()
    }

    /// Structure d'un Chien
    ///
    /// Membre :
    /// * nom : String
    pub struct Chien {
        nom: String,
    }

    /// Structure d'un Chaat
    ///
    /// Membre :
    /// * nom : String
    pub struct Chat {
        nom: String,
    }

    /// Creéation d'une fonction dormir pour le chat
    impl Chat {
        pub fn dormir(&self) {
            println!("Comme tous les chatons, j'aime dormir.")
        }
    }

    /// Création de surcharge pour la structure Chien
    impl Animal for Chien {
        fn creer(nom: String) -> Chien {
            Chien { nom: nom }
        }

        fn emettre_son(&self) -> String {
            "aboiement".to_string()
        }

        fn obtenir_nom(&self) -> String {
            let copie = self.nom.clone();
            copie
        }

        // Surcharge de l'implémentation disponible dans le trait.
        fn afficher(&self) {
            println!(
                "Moi le chien {}, j'émets un {}.",
                self.obtenir_nom(),
                self.emettre_son()
            );
        }
    }

    /// Création de surcharge pour la structure Chat
    impl Animal for Chat {
        fn creer(nom: String) -> Chat {
            Chat { nom: nom }
        }

        fn emettre_son(&self) -> String {
            "miaulement".to_string()
        }

        fn obtenir_nom(&self) -> String {
            let copie = self.nom.clone();
            copie
        }
    }
}
