(module
 (type $none_=>_none (func))
 (type $none_=>_i32 (func (result i32)))
 (type $f32_=>_f32 (func (param f32) (result f32)))
 (import "env" "js_sin" (func $fimport$0 (param f32) (result f32)))
 (memory $0 38)
 (global $global$0 i32 (i32.const 1048580))
 (global $global$1 i32 (i32.const 2488580))
 (global $global$2 i32 (i32.const 2488580))
 (export "memory" (memory $0))
 (export "go" (func $0))
 (export "BUFFER" (global $global$0))
 (export "the_answer" (func $1))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $0
  (local $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 f32)
  (local $6 f32)
  (local $7 f32)
  (local $8 i32)
  (i32.store
   (i32.const 1048576)
   (i32.add
    (local.tee $3
     (i32.load
      (i32.const 1048576)
     )
    )
    (i32.const 1)
   )
  )
  (local.set $1
   (i32.const 1048580)
  )
  (loop $label$1
   (br_if $label$1
    (i32.ne
     (local.tee $0
      (block (result i32)
       (local.set $8
        (i32.add
         (local.get $0)
         (i32.const 1)
        )
       )
       (local.set $7
        (f32.convert_i32_u
         (local.get $0)
        )
       )
       (local.set $0
        (local.get $1)
       )
       (local.set $2
        (i32.const 0)
       )
       (loop $label$2
        (i32.store
         (local.get $0)
         (i32.or
          (i32.add
           (select
            (i32.const 16777215)
            (block $label$3 (result i32)
             (if
              (i32.and
               (f32.lt
                (local.tee $6
                 (select
                  (local.tee $5
                   (f32.add
                    (f32.mul
                     (call $fimport$0
                      (f32.convert_i32_u
                       (local.get $2)
                      )
                     )
                     (f32.const 255)
                    )
                    (f32.mul
                     (call $fimport$0
                      (local.get $7)
                     )
                     (f32.const 255)
                    )
                   )
                  )
                  (f32.const 0)
                  (f32.gt
                   (local.get $5)
                   (f32.const 0)
                  )
                 )
                )
                (f32.const 4294967296)
               )
               (f32.ge
                (local.get $6)
                (f32.const 0)
               )
              )
              (br $label$3
               (i32.trunc_f32_u
                (local.get $6)
               )
              )
             )
             (i32.const 0)
            )
            (f32.gt
             (local.get $5)
             (f32.const 4294967040)
            )
           )
           (local.get $3)
          )
          (i32.const -16777216)
         )
        )
        (local.set $0
         (i32.add
          (local.get $0)
          (i32.const 4)
         )
        )
        (br_if $label$2
         (i32.ne
          (local.tee $2
           (i32.add
            (local.get $2)
            (i32.const 1)
           )
          )
          (i32.const 600)
         )
        )
       )
       (local.set $1
        (i32.add
         (local.get $1)
         (i32.const 2400)
        )
       )
       (local.get $8)
      )
     )
     (i32.const 600)
    )
   )
  )
 )
 (func $1 (result i32)
  (i32.const 42)
 )
)

