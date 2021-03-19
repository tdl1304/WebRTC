# Lag en README.md fil for STUN-serveren. Ta med:  
Navn på STUN-serveren og eventuell lenke til siste continuous integration/deployment kjøring  
Introduksjon  
Implementert funksjonalitet  
Fremtidig arbeid med oversikt over nåværende mangler  
Eksterne avhengigheter med en kort beskrivelse av hver avhengighet og hva den er brukt til  
* [Bincode](https://github.com/bincode-org/bincode)
    * A compact encoder / decoder pair that uses a binary zero-fluff encoding scheme. 
      The size of the encoded object will be the same or smaller than the size that the object takes up in memory in a running Rust program.
    
    * Used for serializing structs into byte arrays
* [Serde](https://docs.serde.rs/serde/index.html) 
    * Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. 
    
    * Used with Bincode to serialize structs into byte arrays

Installasjonsinstruksjoner  
Instruksjoner for å starte STUN-serveren  
Hvordan en kan kjøre eventulle tester  
Eventuell lenke til API dokumentasjon  