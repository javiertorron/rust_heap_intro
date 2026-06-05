
1. **Ubicación Actual:** [Tema 1 -> Capítulo 1.1 -> Sección 1.1.2 -> Apartado: Gestión de Memoria Dinámica (Heap)]
2. **Concepto Clave:** Introducción a la asignación de memoria en el Heap mediante el uso de `Box<T>`.
3. **Prerrequisitos de Estudio:**
   - Diferencias fundamentales entre Stack y Heap (alocación vs. direccionamiento).
   - Concepto de "Smart Pointers" en Rust.
   - Documentación oficial de Rust: `std::boxed::Box`.
4. **Enunciado del Ejercicio:**
   En el ejercicio anterior trabajaste con tipos de tamaño conocido en el Stack. Ahora, debes crear un programa que mueva un valor entero (`i32`) y una estructura simple al Heap.
   
   **Pasos:**
   - Define una estructura llamada `SecretData` que contenga un campo `value: u64`.
   - En la función `main`, instancia un `i32` y un `SecretData` directamente en el Heap usando `Box`.
   - Implementa una función `#[cfg(test)]` que verifique que los valores almacenados en el Heap son correctos y que el tamaño del `Box` en el Stack coincide con el tamaño de un puntero (usando `std::mem::size_of_val`).
   - El test debe fallar si el valor dentro del Box no es el esperado.
