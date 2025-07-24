
-- Eliminar triggers y funciones para AUTOR_DEL_OBJETO
DROP TRIGGER IF EXISTS trg_validar_autor_objeto ON autor_del_objeto;
DROP FUNCTION IF EXISTS validar_autor_objeto();

-- Eliminar triggers y funciones para GENERO_DEL_OBJETO
DROP TRIGGER IF EXISTS trg_validar_genero_objeto ON genero_del_objeto;
DROP FUNCTION IF EXISTS validar_genero_objeto();

-- Eliminar triggers y funciones para OBJETO_EN_COLECCION
DROP TRIGGER IF EXISTS trg_validar_objeto_en_coleccion ON objeto_en_coleccion;
DROP FUNCTION IF EXISTS validar_objeto_en_coleccion();

-- Eliminar triggers y funciones para DISTRIBUIDOR_DEL_OBJETO
DROP TRIGGER IF EXISTS trg_validar_distribuidor_objeto ON distribuidor_del_objeto;
DROP FUNCTION IF EXISTS validar_distribuidor_objeto();
