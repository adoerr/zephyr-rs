/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>


/* External declaration for main in Rust. */
void rust_main(void);

int main(void)
{
	printk("Starting app\n");
	rust_main();
	printk("Done with app\n");
}
