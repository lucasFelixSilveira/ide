#include <stdlib.h>
#include <stdio.h>
#include "uncertain.h"
#include <string.h>

LIB_UNCERTAIN init_uncertain(char *t) {
  LIB_UNCERTAIN result = (LIB_UNCERTAIN) {
    .t = 0,
    ._ = 0,
    .value = NULL
  };

  if( strcmp("[@result]", t) == 0 ) result.t = 0;
  else result.t = 1;

  return result;
}

void uncertain_assert_operator(LIB_UNCERTAIN __struct, char *op) {
  switch(__struct.t) {
    case 0: 
      if( strcmp(op, "Ok") == 0 || strcmp(op, "Err") == 0 ) break;
      else goto __uncertain_error;
    case 1:
      if( strcmp(op, "Some") == 0 || strcmp(op, "None") == 0 ) break;
      else goto __uncertain_error;
    default:
      __uncertain_error: {}
      printf("\nUncertain error: [Operator mismatch]\n- The '%s' operator it's not a valid operator to '%s' type.\n", op, __struct.t == 0 ? "Result" : "Option");
      exit(1);
      break;
  }
}

void uncertain_def_ok(LIB_UNCERTAIN *__struct, void *generic) {
  char t = __struct->t;
  __struct->_ = 1;
  __struct->value = generic;
  __struct->t = t;
}

void uncertain_def_err(LIB_UNCERTAIN *__struct, void *generic) {
  char t = __struct->t;
  __struct->_ = -1;
  __struct->value = generic;
  __struct->t = t;
}

void uncertain_def_some(LIB_UNCERTAIN *__struct, void *generic) {
  char t = __struct->t;
  __struct->_ = 1;
  __struct->value = generic;
  __struct->t = t;
}

void uncertain_def_none(LIB_UNCERTAIN *__struct) {
  char t = __struct->t;
  __struct->_ = -1;
  __struct->t = t;
}

void uncertain_debug(LIB_UNCERTAIN *__struct) {
  printf("LIB_UNCERTAIN { t: %d, _: %d, value: %p }\n", __struct->t, __struct->_, __struct->value);
}