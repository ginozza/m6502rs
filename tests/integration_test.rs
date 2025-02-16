// m6502rs/tests/integration_test.rs

use m6502rs::cpu::CPU;
use m6502rs::memory::{Mem, MAX_MEM};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new();
        // Verifica que los registros se inicialicen en cero.
        assert_eq!(cpu.get_pc(), 0);
        assert_eq!(cpu.get_sp(), 0);
        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_x(), 0);
        assert_eq!(cpu.get_y(), 0);
        assert_eq!(cpu.get_c(), 0);
        assert_eq!(cpu.get_z(), 0);
        assert_eq!(cpu.get_i(), 0);
        assert_eq!(cpu.get_d(), 0);
        assert_eq!(cpu.get_b(), 0);
        assert_eq!(cpu.get_v(), 0);
        assert_eq!(cpu.get_n(), 0);
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        // Para diferenciar, llenamos la memoria con un valor distinto a cero.
        for i in 0..MAX_MEM {
            mem[i] = 0xFF;
        }
        cpu.reset(&mut mem);
        // Tras el reset, se deben cumplir estas condiciones:
        assert_eq!(cpu.get_pc(), 0xFFFC);
        assert_eq!(cpu.get_sp(), 0x0100);
        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_x(), 0);
        assert_eq!(cpu.get_y(), 0);
        assert_eq!(cpu.get_i(), 1); // La bandera de interrupción se pone en 1.
        // Y la memoria debe haberse inicializado (todos ceros).
        for i in 0..MAX_MEM {
            assert_eq!(mem[i], 0);
        }
    }

    #[test]
    fn test_fetch_byte() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        cpu.set_pc(0);
        mem[0] = 0xAB;
        let mut cycles = 5;
        let byte = cpu.fetch_byte(&mut cycles, &mem);
        assert_eq!(byte, 0xAB);
        assert_eq!(cpu.get_pc(), 1);
        assert_eq!(cycles, 4); // Se decrementa 1 ciclo.
    }

    #[test]
    fn test_fetch_word() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        cpu.set_pc(0);
        // Configuramos dos bytes para formar la palabra en little-endian.
        mem[0] = 0xCD; // Byte menos significativo.
        mem[1] = 0xAB; // Byte más significativo.
        let mut cycles = 10;
        let word = cpu.fetch_word(&mut cycles, &mem);
        assert_eq!(word, 0xABCD); // (0xAB << 8) | 0xCD
        assert_eq!(cpu.get_pc(), 2);
        assert_eq!(cycles, 8); // Se han consumido 2 ciclos.
    }

    #[test]
    fn test_read_byte() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        mem[0x42] = 0x99;
        let mut cycles = 3;
        let byte = cpu.read_byte(&mut cycles, 0x42, &mem);
        assert_eq!(byte, 0x99);
        assert_eq!(cycles, 2); // Se decrementa 1 ciclo.
    }

    #[test]
    fn test_lda_set_status() {
        let mut cpu = CPU::new();
        // Caso: acumulador igual a cero.
        cpu.set_a(0);
        cpu.lda_set_status();
        assert_eq!(cpu.get_z(), 1);
        assert_eq!(cpu.get_n(), 0);

        // Caso: acumulador distinto de cero y sin bit negativo.
        cpu.set_a(0x42);
        cpu.lda_set_status();
        assert_eq!(cpu.get_z(), 0);
        assert_eq!(cpu.get_n(), 0);

        // Caso: acumulador con bit 7 (signo) activo.
        cpu.set_a(0x80);
        cpu.lda_set_status();
        assert_eq!(cpu.get_n(), 1);
        assert_eq!(cpu.get_z(), 0);
    }

    #[test]
    fn test_execute_lda_immediate() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        // Ubicamos la instrucción LDA inmediata en la dirección 0.
        cpu.set_pc(0);
        mem[0] = CPU::INS_LDA_IM;
        mem[1] = 0x55;
        let initial_cycles = 10;
        cpu.execute(initial_cycles, &mut mem);
        // Se debe cargar 0x55 en el acumulador.
        assert_eq!(cpu.get_a(), 0x55);
        // Para 0x55, la bandera cero debe estar en 0 y la negativa en 0.
        assert_eq!(cpu.get_z(), 0);
        assert_eq!(cpu.get_n(), 0);
    }

    #[test]
    fn test_execute_lda_zero_page() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        // Ubicamos la instrucción LDA en modo zero page.
        cpu.set_pc(0);
        mem[0] = CPU::INS_LDA_ZP;
        mem[1] = 0x10; // Dirección en zero page.
        mem[0x10] = 0xAA;
        let initial_cycles = 10;
        cpu.execute(initial_cycles, &mut mem);
        assert_eq!(cpu.get_a(), 0xAA);
        // En 0xAA, el bit 7 está activo, por lo que la bandera negativa debe estar en 1.
        assert_eq!(cpu.get_z(), 0);
        assert_eq!(cpu.get_n(), 1);
    }

   #[test]
    fn test_execute_jsr() {
        let mut cpu = CPU::new();
        let mut mem = Mem::new();
        // Para probar JSR usamos reset, que establece pc = 0xFFFC y sp = 0x0100.
        cpu.reset(&mut mem);
        // Colocamos la instrucción JSR en la dirección de reset (0xFFFC).
        mem[0xFFFC] = CPU::INS_JSR;
        // Queremos saltar a la dirección 0x1234.
        // Recordar: en little-endian, primero el byte menos significativo.
        mem[0xFFFD] = 0x34; // Low byte
        mem[0xFFFE] = 0x12; // High byte
        let cycles = 15; // Suficientes ciclos para completar la instrucción.
        cpu.execute(cycles, &mut mem);
        // Luego de la instrucción, se espera que:
        // - El contador de programa (pc) sea 0x1234.
        // - El puntero de pila (sp) se haya decrementado en 1: de 0x0100 a 0x00FF.
        assert_eq!(cpu.get_pc(), 0x1234);
        assert_eq!(cpu.get_sp(), 0x00FF);
        // Además, se debe haber escrito la dirección de retorno (pc - 1) en la pila.
        // Tras JSR, el pc ya se incrementó a 0xFFFF (después de extraer los dos bytes),
        // por lo que el valor a guardar es 0xFFFE (0xFFFF - 1) escrito en memoria en 0x0100.
        // En little-endian: low byte en mem[0x0100] y high byte en mem[0x0101].
        assert_eq!(mem[0x0100], 0xFE);
        assert_eq!(mem[0x0101], 0xFF);
    } 

    #[test]
    fn test_mem_initialize() {
        let mut mem = Mem::new();
        // Establecemos algunos valores arbitrarios.
        mem[1234] = 0xAA;
        mem[4321] = 0xBB;
        // Llamamos a initialize para limpiar la memoria.
        mem.initialize();
        // Verificamos que cada posición en la memoria sea 0.
        for i in 0..MAX_MEM {
            assert_eq!(mem[i], 0);
        }
    }

    #[test]
    fn test_write_word() {
        let mut mem = Mem::new();
        let mut cycles = 10;
        // Escribimos la palabra 0xBEEF en la dirección 1000.
        mem.write_word(0xBEEF, 1000, &mut cycles);
        // En formato little-endian, se espera que:
        // - mem[1000] = byte menos significativo (0xEF)
        // - mem[1001] = byte más significativo (0xBE)
        assert_eq!(mem[1000], 0xEF);
        assert_eq!(mem[1001], 0xBE);
        // Se deben haber consumido 2 ciclos.
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_index_and_index_mut() {
        let mut mem = Mem::new();
        mem[2000] = 0x55;
        assert_eq!(mem[2000], 0x55);
    }
}

